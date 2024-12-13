<script lang="ts">
    import { Github, ExternalLink, Book, CircleHelp } from "lucide-svelte";
    import { base } from "$app/paths";

    let githubHovered = false;
    let moreExamplesHovered = false;
    let docsHovered = false;
</script>

<div class="flex w-full max-w-screen h-screen flex-col">
    <div class="h-fit bg-ctp-mantle text-center pt-2 pb-6 border-b border-b-ctp-surface0">
        <h1 class="text-ctp-yellow text-8xl font-bold [text-shadow:_0_0_10px_#f9e2af] mb-4">Modu</h1>
        <a 
            class="flex w-fit mx-auto bg-white p-2 rounded-md text-ctp-mantle font-semibold transition-all duration-300" 
            href="https://github.com/cyteon/modu" target="_blank" 
            on:mouseenter={() => githubHovered = true} 
            on:mouseleave={() => githubHovered = false}
        >
            <Github size={32} class="my-auto" />
            <span class="ml-1 text-xl my-auto">GitHub</span>
            <ExternalLink size={20} class={`my-auto  ${githubHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
        </a>
    </div>

    <div class="flex flex-col px-2 mx-auto">
        <div class="text-center mx-auto mt-8 bg-ctp-mantle p-4 rounded-md">
            <pre class="text-lg whitespace-pre-wrap">
Modu is a simple and fast interpreted programming language. 
It is designed to be easy to learn and use like python,
but fast and efficient like Rust.
With streamlined error messages for easy debugging</pre>
        </div>

        <div class="my-4 mx-auto space-x-4 flex">
            <a 
                href={base + "/examples"}
                class="bg-ctp-blue text-ctp-crust text-xl p-2 rounded-md text-center w-fit flex"
                on:mouseenter={() => moreExamplesHovered = true}
                on:mouseleave={() => moreExamplesHovered = false}
            >
                <CircleHelp size={20} class="my-auto" />
                <span class="ml-1 my-auto">More Examples</span>
                <ExternalLink size={20} class={`my-auto ${moreExamplesHovered ? "ml-1 w-6" : "w-0"} transition-all duration-300`} />
            </a>

            <a 
                href={base + "/docs"}
                class="bg-ctp-pink text-ctp-crust text-xl p-2 rounded-md text-center w-fit flex"
                on:mouseenter={() => docsHovered = true}
                on:mouseleave={() => docsHovered = false}
            >
                <Book size={20} class="my-auto" />
                <span class="ml-1 my-auto">Docs</span>
                <ExternalLink size={20} class={`my-auto ${docsHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
            </a>
        </div>

        <div class="mx-auto mb-2 bg-ctp-mantle p-4 rounded-md">
            <pre class="text-lg whitespace-pre-wrap break-all">
<span class="text-ctp-red">let</span> x = <span class="text-ctp-peach">10.5</span>;
<span class="text-ctp-red">let</span> y = x;
<span class="text-ctp-pink">print</span>(x);
<span class="text-ctp-pink">print</span>(y);
<span class="text-ctp-pink">print</span>(z);
                
<span class="text-ctp-subtext0">// Output</span>
                
10.5
10.5
                
⚠️ Variable x not found
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