<main>
  <div id="main">
    <button>Sporta</button>
    <div id="img"></div>
  </div>
</main>

<style>
    main {
        width: 1280px;
        height: 720px;
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
        height: 250px;
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
    }
</style>
