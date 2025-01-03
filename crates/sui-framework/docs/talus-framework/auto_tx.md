---
title: Module `0xa070::auto_tx`
---



-  [Resource `AutoTx`](#0xa070_auto_tx_AutoTx)
-  [Function `create_auto_tx`](#0xa070_auto_tx_create_auto_tx)


<pre><code><b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xa070_auto_tx_AutoTx"></a>

## Resource `AutoTx`

Definition of the <code><a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a></code> struct


<pre><code><b>struct</b> <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>trigger_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>caller: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>callee: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>module_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>function_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>type_inputs: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>gas_id: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>arguments: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xa070_auto_tx_create_auto_tx"></a>

## Function `create_auto_tx`

Entry function to create an instance of the <code><a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a></code> struct


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_auto_tx">create_auto_tx</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, trigger_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, caller: <b>address</b>, callee: <b>address</b>, module_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, function_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, type_inputs: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, gas: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, arguments: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_auto_tx">create_auto_tx</a>(
    ctx: &<b>mut</b> TxContext,
    trigger_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    caller: <b>address</b>,
    callee: <b>address</b>,
    module_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    function_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    type_inputs: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    gas: Coin&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    arguments: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
): () {
    // Transfer the gas (<a href="../sui-framework/coin.md#0x2_coin">coin</a>) <b>to</b> the package <b>address</b>
    <b>let</b> gas_id = <a href="../sui-framework/object.md#0x2_object_id_to_address">object::id_to_address</a>(&<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&gas));
    public_transfer(gas, address::from_u256(0xa070));

    // Create the <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> <a href="../sui-framework/object.md#0x2_object">object</a>
    <b>let</b> <a href="auto_tx.md#0xa070_auto_tx">auto_tx</a> = <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        trigger_time,
        caller,
        callee,
        module_name,
        function_name,
        type_inputs,
        gas_id: gas_id,
        arguments:arguments,
    };

    // Transfer the <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> <a href="../sui-framework/object.md#0x2_object">object</a> <b>to</b> the package <b>address</b>
    public_transfer(<a href="auto_tx.md#0xa070_auto_tx">auto_tx</a>, address::from_u256(0xa070));
}
</code></pre>



</details>
