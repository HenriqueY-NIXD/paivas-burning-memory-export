<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';

    async function sporta() {
        const folderPath = await open({directory: true});
        await invoke('sporta', folderPath)
    }
</script>

<main>
  <div id="main">
    <button on:click="{sporta}">Sporta</button>
    <div id="img"></div>
  </div>
</main>

<style>
    main {
        width: 1200px;
        height: 700px;
        display: flex;
        flex: 1;
        justify-content: center;
        align-items: center;
    }

    button {
        background: #4b4b4b;
        border: solid 1px #fff;
        outline: none;
        box-shadow: none;
        cursor: pointer;
        width: 500px;
        height: 100px;
        font-size: larger;
        font-weight: bolder;
        color: #fff;
        border: solid black 1px;
    }

    button:hover {
        background: #414141;
    }

    button:active {
        background: #313131;
        cursor: grab;
    }

    #img {
        box-sizing: border-box;
        background-color: #4b4b4b;
        border: solid 1px #fff;
        height: 500px;
        width: 500px;
        background-image: url("/god.png");
        background-size: 500px 500px;
    }
</style>
