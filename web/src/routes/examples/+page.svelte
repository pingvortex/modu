<script>
    import { Home } from 'lucide-svelte';
    import { base } from '$app/paths';
</script>

<svelte:head>
    <meta name="title" content="Modu Examples" />
    <meta name="description" content="Examples of the Modu programming language." />
</svelte:head>

<div class="flex flex-col w-full">
    <div class="mx-auto">
        <h1 class="mt-4 w-full font-bold text-7xl">Examples</h1>

        <a href={base + "/"} class="mx-auto mt-5 bg-ctp-blue text-ctp-crust text-xl p-2 rounded-md text-center w-fit flex">
            <Home size={20} class="my-auto" />
            <span class="ml-1 my-auto">Return Home</span>
        </a>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
        <div class="w-full p-4">
            <div class="bg-ctp-mantle h-full p-4 rounded-md">
                <h2>Variables</h2>
                <pre class="text-lg whitespace-pre-wrap">
<span class="text-ctp-red">let</span> a = <span class="text-ctp-green   ">"Hello, World!"</span>;
<span class="text-ctp-red">let</span> b = <span class="text-ctp-peach">1.5</span>;
<span class="text-ctp-red">let</span> c = <span class="text-ctp-green">true</span>;
<span class="text-ctp-red">let</span> d = c;

<span class="text-ctp-pink">print</span>(a);
<span class="text-ctp-pink">print</span>(b);
<span class="text-ctp-pink">print</span>(c);
<span class="text-ctp-pink">print</span>(d);

<span class="text-ctp-subtext0">// Output</span>

Hello, World!
1.5
true
true</pre>
            </div>
        </div>

        <div class="w-full p-4">
            <div class="bg-ctp-mantle h-full p-4 rounded-md">
                <h2>Functions</h2>
                <pre class="text-lg whitespace-pre-wrap">
<span class="text-ctp-red">fn</span> {@html `yap(str) {
    <span class="text-ctp-pink">print</span>(str);
}`}

<span class="text-ctp-pink">yap</span>(<span class="text-ctp-green">"Hello, World!"</span>);

<span class="text-ctp-subtext0">// Output</span>

Hello, World!</pre>
            </div>
        </div> 
        <div class="w-full p-4">
            <div class="bg-ctp-mantle h-full p-4 rounded-md">
                <h2>Imports</h2>
                <pre class="text-lg whitespace-pre-wrap">

<span class="text-ctp-subtext0">// yapper.modu</span>

<span class="text-ctp-red">fn</span> {@html `yap(str) {
    <span class="text-ctp-pink">print</span>(str);
}`}

<span class="text-ctp-subtext0">// main.modu</span>

<span class="text-ctp-red">import</span> <span class="text-ctp-green">"yapper.modu"</span> as <span class="text-ctp-pink">yapper</span>;

yapper.<span class="text-ctp-pink">yap</span>(<span class="text-ctp-green">"Hello, World!"</span>);

<span class="text-ctp-subtext0">// Output</span>

Hello, World!</pre>
            </div>
        </div> 
        
        <div class="w-full p-4">
            <div class="bg-ctp-mantle h-full p-4 rounded-md">
                <h2>If Statements</h2>
                <pre class="text-lg whitespace-pre-wrap">
<span class="text-ctp-red">let</span> a = <span class="text-ctp-peach">1</span>;
<span class="text-ctp-red">let</span> b = <span class="text-ctp-peach">2</span>;

<span class="text-ctp-red">if</span> a == b {@html `{
    <span class="text-ctp-pink">print</span>(<span class="text-ctp-green">"a is equal to b"</span>);
}
`}

<span class="text-ctp-red">if</span> a != b {@html `{
    <span class="text-ctp-pink">print</span>(<span class="text-ctp-green">"a is not equal to b"</span>);
}
`}

<span class="text-ctp-subtext0">// Output</span>

a is not equal to b</pre>
            </div>
        </div>  

        <div class="w-full p-4">
            <div class="bg-ctp-mantle h-full p-4 rounded-md">
                <h2>Addition and Subtraction</h2>
                <pre class="text-lg whitespace-pre-wrap">
<span class="text-ctp-red">let</span> a = <span class="text-ctp-peach">5</span>;
<span class="text-ctp-red">let</span> b = <span class="text-ctp-peach">-5</span>;
<span class="text-ctp-red">let</span> c = a + b;

<span class="text-ctp-pink">print</span>(a);
<span class="text-ctp-pink">print</span>(b);
<span class="text-ctp-pink">print</span>(c);
<span class="text-ctp-pink">print</span>(a + b + c + 1);

<span class="text-ctp-subtext0">// Output</span>

5
-5
0
1</pre>
            </div>
        </div> 
    </div>
</div>

<style>
    h2 {
        @apply text-2xl font-semibold my-1 text-center;
    }
</style>