---
title: Module `0xa070::auto_tx`
---



-  [Struct `ObjectRef`](#0xa070_auto_tx_ObjectRef)
-  [Resource `AutoTx`](#0xa070_auto_tx_AutoTx)
-  [Enum `TypeTag`](#0xa070_auto_tx_TypeTag)
-  [Enum `CallArg`](#0xa070_auto_tx_CallArg)
-  [Constants](#@Constants_0)
-  [Function `create_type`](#0xa070_auto_tx_create_type)
-  [Function `minimum_gas`](#0xa070_auto_tx_minimum_gas)
-  [Function `create_pure_Arg`](#0xa070_auto_tx_create_pure_Arg)
-  [Function `create_object_Arg`](#0xa070_auto_tx_create_object_Arg)
-  [Function `create_auto_tx`](#0xa070_auto_tx_create_auto_tx)
-  [Function `drop_auto_tx`](#0xa070_auto_tx_drop_auto_tx)


<pre><code><b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../sui-framework/bcs.md#0x2_bcs">0x2::bcs</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/sui.md#0x2_sui">0x2::sui</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xa070_auto_tx_ObjectRef"></a>

## Struct `ObjectRef`



<pre><code><b>struct</b> <a href="auto_tx.md#0xa070_auto_tx_ObjectRef">ObjectRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>mutable: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>receiving: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

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

<a name="0xa070_auto_tx_TypeTag"></a>

## Enum `TypeTag`



<pre><code><b>public</b> enum TypeTag <b>has</b> drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Bool</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U8</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U64</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U128</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>Address</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>Signer_not_supported</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>Vector_not_supported</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>Struct_not_supported</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U16</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U32</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>U256</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xa070_auto_tx_CallArg"></a>

## Enum `CallArg`



<pre><code><b>public</b> enum CallArg <b>has</b> drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Pure</code>
</dt>
<dd>

</dd>

<dl>
<dt>
<code>pos0: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>

<dt>
Variant <code>Object</code>
</dt>
<dd>

</dd>

<dl>
<dt>
<code>pos0: <a href="auto_tx.md#0xa070_auto_tx_ObjectRef">auto_tx::ObjectRef</a></code>
</dt>
<dd>

</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xa070_auto_tx_ArgMismatch"></a>



<pre><code>#[error]
<b>const</b> <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Input <a href="../sui-framework/types.md#0x2_types">types</a> and arguments mismatch";
</code></pre>



<a name="0xa070_auto_tx_InvalidCaller"></a>



<pre><code>#[error]
<b>const</b> <a href="auto_tx.md#0xa070_auto_tx_InvalidCaller">InvalidCaller</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid Caller";
</code></pre>



<a name="0xa070_auto_tx_InvalidDelete"></a>



<pre><code>#[error]
<b>const</b> <a href="auto_tx.md#0xa070_auto_tx_InvalidDelete">InvalidDelete</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Cannot Delete Not Triggered <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a>";
</code></pre>



<a name="0xa070_auto_tx_InvalidGas"></a>



<pre><code>#[error]
<b>const</b> <a href="auto_tx.md#0xa070_auto_tx_InvalidGas">InvalidGas</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Need <b>to</b> reclaim the exact gas <a href="../sui-framework/object.md#0x2_object">object</a>";
</code></pre>



<a name="0xa070_auto_tx_create_type"></a>

## Function `create_type`



<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_type">create_type</a>(bits: u8): auto_tx::TypeTag
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_type">create_type</a>(bits: u8): TypeTag {
  match (bits) {
    1 =&gt; TypeTag::Bool,
    3 =&gt; TypeTag::U8,
    4 =&gt; TypeTag::U16,
    5 =&gt; TypeTag::U32,
    6 =&gt; TypeTag::U64,
    7 =&gt; TypeTag::U128,
    8 =&gt; TypeTag::U256,
    _ =&gt; TypeTag::Address,
  }
}
</code></pre>



</details>

<a name="0xa070_auto_tx_minimum_gas"></a>

## Function `minimum_gas`



<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_minimum_gas">minimum_gas</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_minimum_gas">minimum_gas</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
  10_000_000
}
</code></pre>



</details>

<a name="0xa070_auto_tx_create_pure_Arg"></a>

## Function `create_pure_Arg`



<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_pure_Arg">create_pure_Arg</a>(data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): auto_tx::CallArg
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_pure_Arg">create_pure_Arg</a>(data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): CallArg {
  CallArg::Pure(data)
}
</code></pre>



</details>

<a name="0xa070_auto_tx_create_object_Arg"></a>

## Function `create_object_Arg`



<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_object_Arg">create_object_Arg</a>(id: <b>address</b>, mutable: bool, receiving: bool): auto_tx::CallArg
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_object_Arg">create_object_Arg</a>(id: <b>address</b>, mutable: bool, receiving: bool): CallArg {
  CallArg::Object(<a href="auto_tx.md#0xa070_auto_tx_ObjectRef">ObjectRef</a> {
    id,
    mutable,
    receiving,
  })
}
</code></pre>



</details>

<a name="0xa070_auto_tx_create_auto_tx"></a>

## Function `create_auto_tx`

function to create an instance of the <code><a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a></code> struct


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_auto_tx">create_auto_tx</a>(trigger_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, caller: <b>address</b>, callee: <b>address</b>, module_name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, function_name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, type_inputs: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;auto_tx::TypeTag&gt;, gas: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, arguments: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;auto_tx::CallArg&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_create_auto_tx">create_auto_tx</a>(
  trigger_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
  caller: <b>address</b>,
  callee: <b>address</b>,
  module_name: String,
  function_name: String,
  type_inputs: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;TypeTag&gt;,
  gas: Coin&lt;SUI&gt;,
  arguments: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;CallArg&gt;,
  ctx: &<b>mut</b> TxContext,
) {
  // Transfer the gas (<a href="../sui-framework/coin.md#0x2_coin">coin</a>) <b>to</b> the package <b>address</b>
  <b>let</b> gas_id = <a href="../sui-framework/object.md#0x2_object_id_to_address">object::id_to_address</a>(&<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&gas));
  public_transfer(gas, caller);
  // Ensure type_inputs and arguments have a one-<b>to</b>-one correspondence
  <b>let</b> len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&type_inputs);

  // Ensure type_inputs and arguments have a one-<b>to</b>-one correspondence
  <b>assert</b>!(len== <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&arguments), <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>);
  <b>let</b> <b>mut</b> i = 0;
  <b>while</b> (i &lt; len) {
    <b>let</b> input = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&type_inputs, i);
    <b>let</b> arg = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&arguments, i);
    match (arg) {
      CallArg::Pure(_) =&gt; {
        match (input) {
          TypeTag::Vector_not_supported =&gt; { <b>assert</b>!(<b>false</b>, <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>); },
          TypeTag::Struct_not_supported =&gt; { <b>assert</b>!(<b>false</b>, <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>); },
          TypeTag::Signer_not_supported =&gt; { <b>assert</b>!(<b>false</b>, <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>); },
          _ =&gt; {},
        }
      },
      CallArg::Object(_) =&gt; {
        match (input) {
          TypeTag::Address =&gt; {},
          _ =&gt; { <b>assert</b>!(<b>false</b>, <a href="auto_tx.md#0xa070_auto_tx_ArgMismatch">ArgMismatch</a>); },
        }
      },
    };
    i = i + 1;
  };

  // Create the <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> <a href="../sui-framework/object.md#0x2_object">object</a>
  <b>let</b> <a href="auto_tx.md#0xa070_auto_tx">auto_tx</a> = <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> {
    id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    trigger_time,
    caller,
    callee,
    module_name: *<a href="../move-stdlib/string.md#0x1_string_as_bytes">string::as_bytes</a>(&module_name),
    function_name: *<a href="../move-stdlib/string.md#0x1_string_as_bytes">string::as_bytes</a>(&function_name),
    type_inputs: <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&type_inputs),
    gas_id: gas_id,
    arguments: <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&arguments),
  };

  // Transfer the <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> <a href="../sui-framework/object.md#0x2_object">object</a> <b>to</b> the package <b>address</b>
  public_transfer(<a href="auto_tx.md#0xa070_auto_tx">auto_tx</a>, @<a href="talus.md#0xa070_talus">talus</a>);
}
</code></pre>



</details>

<a name="0xa070_auto_tx_drop_auto_tx"></a>

## Function `drop_auto_tx`

function to create an instance of the <code><a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a></code> struct


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_drop_auto_tx">drop_auto_tx</a>(obj: <a href="auto_tx.md#0xa070_auto_tx_AutoTx">auto_tx::AutoTx</a>, remain: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="auto_tx.md#0xa070_auto_tx_drop_auto_tx">drop_auto_tx</a>(obj: <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a>, remain: Coin&lt;SUI&gt;, ctx: &<b>mut</b> TxContext): () {
  // TODO: add cancel logic for deleting not triggered <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a>, and reclaim gas.
  <b>let</b> epoch_start = ctx.epoch_timestamp_ms();
  <b>assert</b>!(epoch_start&gt;obj.trigger_time, <a href="auto_tx.md#0xa070_auto_tx_InvalidDelete">InvalidDelete</a>);

  // claim back remaining gas
  <b>let</b> gas_id = <a href="../sui-framework/object.md#0x2_object_id_to_address">object::id_to_address</a>(&<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&remain));
  <b>assert</b>!(gas_id == obj.gas_id, <a href="auto_tx.md#0xa070_auto_tx_InvalidGas">InvalidGas</a>);

  // only caller can delete
  <b>assert</b>!(obj.caller == ctx.sender(), <a href="auto_tx.md#0xa070_auto_tx_InvalidCaller">InvalidCaller</a>);

  // <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> gas back <b>to</b> caller
  public_transfer(remain, obj.caller);

  <b>let</b> <a href="auto_tx.md#0xa070_auto_tx_AutoTx">AutoTx</a> { id, .. } = obj;
  <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>
