<script lang="ts">
    import { Github, ExternalLink } from "lucide-svelte";

    let githubHovered = false;
</script>

<div class="flex w-full max-w-screen h-screen flex-col">
    <div class="h-fit bg-ctp-mantle text-center pt-6 pb-8 border-b border-b-ctp-surface0">
        <h1 class="text-ctp-yellow text-8xl font-bold [text-shadow:_0_0_10px_#f9e2af] mb-4">Modu</h1>
        <a 
            class="flex w-fit mx-auto bg-white p-2 rounded-md text-ctp-mantle font-semibold transition-all duration-300" 
            href="https://github.com/cyteon/modu" target="_blank" 
            on:mouseenter={() => githubHovered = true} 
            on:mouseleave={() => githubHovered = false}
        >
            <Github size={32} class="my-auto" />
            <span class="ml-1 text-xl my-auto">GitHub</span>
            <ExternalLink size={20} class={`my-auto ml-1 ${githubHovered ? "w-6" : "w-0"} transition-all duration-300 overflow-hidden`} />
        </a>
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