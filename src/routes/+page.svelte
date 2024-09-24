<script lang="js">
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';

    // type Data = {
    //     album: string;
    //     artist_name: string;
    //     created_at: string;
    //     listen_at: string;
    //     order: number;
    //     photo: string;
    // }

    let isError = false;
    let isSucess = false;

    async function sporta() {
        const folderPath = await open({directory: true});
        if (!folderPath) return;

        invoke('sporta')
        .then((data) => {
            isSucess = true;

            setTimeout(() => {
                isSucess = false;
            }, 2000);
        })
        .catch(() => {
            isError = true;

            setTimeout(() => {
                isError = false;
            }, 2000);
        });
    }
</script>

<main>
  <div id="main">
    <button on:click="{sporta}">Sporta</button>
    <div id="img"></div>
  </div>
</main>

{#if isError && !isSucess}
<div id="erro" class="toast">Não foi possível salvar o arquivo!</div>
{/if}

{#if !isError && isSucess}
<div id="sucesso" class="toast">Arquivo Salvo!</div>
{/if}

<style>
    #sucesso {
        background-color: #1fe419;
    }

    #erro {
        background-color: #ff3434;
    }

    .toast {
        min-width: 250px;
        margin-left: -125px;
        color: #fff;
        text-align: center;
        border-radius: 2px;
        padding: 16px;
        position: fixed;
        z-index: 1;
        right: 2%;
        bottom: 30px;
        font-size: 17px;
        -webkit-animation: fadein 0.5s, fadeout 0.5s 2.5s;
        animation: fadein 0.5s, fadeout 0.5s 2.5s;
    }

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
