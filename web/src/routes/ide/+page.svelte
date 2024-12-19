<script lang="ts">
    import { PUBLIC_VITE_IDE_BACKEND } from "$env/static/public";
    import { Play } from "lucide-svelte";
    import { basicSetup, EditorView } from "codemirror";
    import { EditorState, Compartment } from "@codemirror/state"
    import { rust } from "@codemirror/lang-rust";
    import {tags} from "@lezer/highlight"
    import { HighlightStyle, syntaxHighlighting } from "@codemirror/language"
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import { base } from "$app/paths";

    let language = new Compartment, tabsize = new Compartment;

    let state = EditorState.create({
        doc: "print(\"Hello, World!\");\n\n\n\n\n\n\n\n\n",
        extensions: [
            basicSetup,
            language.of(rust()),
            tabsize.of(EditorState.tabSize.of(4)),
            EditorView.theme({
                "&": {
                    color: "#cdd6f4",
                    backgroundColor: "#11111b",
                    fontSize: "24px",
                },

                "&.cm-focused": {
                    outline: "none",
                },

                ".cm-activeLine": {
                    backgroundColor: "#89b4fa10",
                },

                ".cm-gutters": {
                    backgroundColor: "#181825",
                },

            }, { dark: true }),
            syntaxHighlighting(HighlightStyle.define([
                { tag: tags.string, color: "#a6e3a1" },
                { tag: tags.keyword, color: "#cba6f7" },
                { tag: tags.atom, color: "#f38ba8" },
                { tag: tags.escape, color: "#f5c2e7" },
                { tag: tags.comment, color: "#9399b2" },
                { tag: tags.number, color: "#fab387" },
                { tag: tags.float, color: "#fab387" },
                { tag: tags.operator, color: "#89dceb" },
                { tag: tags.brace, color: "#9399b2" },
                { tag: tags.bool, color: "#89b4fa" }
            ])),
        ]
    });

    let view;

    onMount(() => {
        if (browser) {
            view = new EditorView({
                state,
                parent: document.querySelector("#code"),
            });
        }
    })

    let output = "";

    async function run() {
        const res = await fetch(PUBLIC_VITE_IDE_BACKEND + "/eval", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: view.state.doc.toString(),
        });

        output = (await res.text()).trim();

        if (output == "") {
            output = "No output";
        }
    }
</script>

<div class="flex flex-col w-full h-screen">
    <div class="w-full border-b border-b-ctp-surface0 p-2 px-4 flex">
        <a href={base + "/"} class="text-3xl font-bold inline-block bg-clip-text text-transparent bg-gradient-to-r from-ctp-red to-75% to-ctp-yellow">Modu</a>

        <div class="ml-auto my-auto">
            <a href="docs" class="text-2xl">Docs</a>
        </div>

        <div class="ml-auto flex">
            <button class="bg-ctp-blue text-ctp-crust px-2 rounded-md my-auto text-center font-mono w-fit flex" on:click={run}>
                <Play size={20} class="my-auto" />
                <span class="ml-1 text-xl mt-0.5">Run</span>
            </button>
        </div>
    </div>

    <div class="flex p-4 h-full space-y-4 flex-col md:flex-row md:space-x-4 md:space-y-0">
        <div class="bg-ctp-mantle w-full p-4 h-full rounded-md flex flex-col md:w-2/3">
            <h1 class="text-3xl font-bold">Input</h1>
            <div id="code" class="mt-2"></div>
        </div>

        <div class="bg-ctp-mantle w-full p-4 h-full rounded-md flex flex-col md:w-1/3">
            <h1 class="text-3xl font-bold">Output</h1>
            <!-- pt-4 instead of mt-4 cause smth broke -->
            <pre class="bg-ctp-mantle pt-4 text-xl break-words whitespace-pre-wrap">{output}</pre>
        </div>
    </div>
</div>