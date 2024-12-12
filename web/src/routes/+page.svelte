<div class="flex w-full max-w-screen h-screen flex-col">
    <div class="h-fit bg-ctp-mantle text-center pt-6 pb-8 border-b border-b-ctp-surface0">
        <h1 class="text-ctp-yellow text-8xl font-bold [text-shadow:_0_0_10px_#f9e2af]">Modu</h1>
    </div>

    <div class="flex flex-col px-2">
        <div class="text-center mx-auto mt-8 bg-ctp-mantle p-4 rounded-md">
            <pre class="text-lg whitespace-pre-wrap">
Modu is a simple and fast interpreted programming language. 
It is designed to be easy to learn and use like python,
but fast and efficient like Rust.
With streamlined error messages for easy debugging</pre>
        </div>

        <div class="mx-auto mt-8 bg-ctp-mantle p-4 rounded-md">
            <pre class="text-lg whitespace-pre-wrap break-all">
<span class="text-ctp-red">let</span> x = <span class="text-ctp-blue">10.5</span>;
<span class="text-ctp-red">let</span> y = <span class="text-ctp-blue">x</span>;
<span class="text-ctp-pink">print</span>(x);
<span class="text-ctp-pink">print</span>(y);
<span class="text-ctp-pink">print</span>(z);
                
<span class="text-ctp-subtext0">// Output</span>
                
10.5
10.5
                
⚠️ Unknown variable: z
Traceback (most recent call last):
    File "file.modu", line 6
─────┬──────────────────────────────────────────────────────
     │ File: file.modu
─────┼──────────────────────────────────────────────────────
   5 │ print(y);
   6 │ print(z);
─────┴──────────────────────────────────────────────────────</pre>
        </div>
    </div>
</div>