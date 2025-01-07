#!/bin/bash

#This script is used to test the catch_ball example.
SUI="../../../target/debug/sui"
# Run a command in the background.
_evalBg() {
    eval "$@" &>/dev/null & disown;
}

_getCoins() {
    data=$($SUI client gas | awk -F '│' '/0x/ {gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')

    counter=1

    # Loop through each line
    while IFS= read -r line; do
        eval "coinId_$counter=\"$line\""
        counter=$((counter + 1))
    done <<< "$data"

    # Print variables to verify
    for i in $(seq 1 $((counter - 1))); do
        eval "echo Coin ID \$i: \$coinId_$i"
    done

}


# echo "Start Node"
# start_node="RUST_LOG=\"off,sui_node=error\" $SUI start --with-faucet --force-regenesis";
# _evalBg "${start_node}";

echo "Waiting for the node to start"
sleep 15

echo "setup client"
$SUI client new-env --alias local --rpc http://127.0.0.1:9000
$SUI client switch --env local
$SUI client active-env

echo "get address"
USER=$($SUI client active-address)
echo "Active address: \"$USER\""

echo "get faucet"
$SUI client faucet

sleep 5

_getCoins 

echo "Publish Challenger:"
ChallengerContractID=$($SUI client publish --gas-budget 20000000 ./challenger --json | sed -n 's/.*"packageId": "\([^"]*\)".*/\1/p')

sleep 5
echo "Challenger contract at: \"$ChallengerContractID\""

echo "Publish Responser:"
ResponserContractID=$($SUI client publish --gas-budget 20000000 ./responser --json  | sed -n 's/.*"packageId": "\([^"]*\)".*/\1/p')
sleep 5
echo "Responser contract at: \"$ResponserContractID\""

echo "Initiate Responser"

ResponserID=$($SUI client call --package $ResponserContractID --module responser --function initiate  --args $coinId_3 --json | jq -r ".effects.created.[0].reference.objectId")

sleep 5
echo "Responser at: \"$ResponserID\""

echo "generate bet:"
$SUI client split-coin --coin-id  $coinId_1 --count 10 > /dev/null 2>&1
$SUI client split-coin --coin-id  $coinId_4 --count 5 > /dev/null 2>&1

sleep 5
echo "done"

$SUI client call --package $ChallengerContractID --module challenger --function create_challenge --args $ResponserContractID --args $ResponserID --args responser --args response --args $coinId_1 --args $coinId_4 --dry-run
