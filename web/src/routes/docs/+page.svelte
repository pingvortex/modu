<script>
    import { goto } from "$app/navigation";
    import { browser } from "$app/environment";
    import { base } from "$app/paths";
    import { onMount } from "svelte";

    onMount(() => {
        goto(base + "/docs/quickstart");
    });
</script>