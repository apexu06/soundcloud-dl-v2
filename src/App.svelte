<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  async function test() {
    await invoke("download_track", {
      url: "https://soundcloud.com/owrry/my-war-levitating-attack-on-titan-x-dua-lipa-mashup",
    });
  }
  onMount(async () => {
    // const response = await scdl.download(SOUNDCLOUD_URL);
    // console.log(response);
  });
</script>

<main class="container">
  <button on:click={async () => await test()}>download</button>
</main>
