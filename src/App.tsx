import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [searchResult, setSearchResult] = useState("");
  const [gameName, setGameName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


    let dirsToCheck = [
      "C:/Program Files (x86)/Steam/steamapps/common",
      "P:/SteamLibrary/steamapps/common/",
      "C:/Users/Michael/AppData/LocalLow",
      "C:/Users/Michael/AppData/Roaming",
      "C:/Users/Michael/AppData/Local",
      "C:/Users/Michael/Saved Games",
      "C:/Users/Michael/Documents",
      "D:/My Installed Games/Steam Games/steamapps/common",
      "D:/My Documents",
    ]

    console.log(gameName, dirsToCheck)

    let saveLocation: string = await invoke("find_game_save_path", { gameName, dirsToCheck })

    setSearchResult(saveLocation);
  }

  return (
    <div className="container">
      <h1>Game Save Finder</h1>

      <p>Type in the Game name that you want to find the save location for.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setGameName(e.currentTarget.value)}
          placeholder="Enter a game name."
        />
        <button type="submit">Save Search</button>
      </form>

      <p>{searchResult}</p>
    </div>
  );
}

export default App;
