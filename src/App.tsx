import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [searchResult, setSearchResult] = useState<string[]>([]);
  const [gameName, setGameName] = useState("");

  async function findSave() {
    // TODO switch to using a file for this
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

    if (gameName) {
      let saveLocation: string[] = await invoke("find_game_save_paths", { gameName, dirsToCheck })
      setSearchResult(saveLocation);
    } else {
      setSearchResult([]);
    }
  }

  return (
    <div className="container">
      <h1>Game Save Finder</h1>

      <p>Type in the Game name that you want to find the save location for.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          findSave();
        }}
      >
        <input
          id="game-input"
          onChange={(e) => setGameName(e.currentTarget.value)}
          placeholder="Enter Game Name"
        />
        <button type="submit">Save Search</button>
      </form>

      {searchResult.length === 0 ? (
        <div>
          <h3>No Save Paths to Show.</h3>
          <p>Try searching for a game's save path.</p>
        </div>
      ) : (
        // TODO make this scrollable
        <div className="scroll">
          {searchResult.map((path, index) => (
            <p key={index}>{index+1}. {path}</p>
          ))}
        </div>
      )}

    </div>
  );
}

export default App;
