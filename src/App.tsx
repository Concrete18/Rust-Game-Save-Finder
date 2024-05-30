import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface PossiblePath {
  path: string,
  score: number
}

function App() {
  const [searchResult, setSearchResult] = useState<PossiblePath[]>([]);
  const [gameName, setGameName] = useState("");

  const [selectedValue, setSelectedValue] = useState('');

  const handleRadioChange = (e: any) => {
    const value = e.target.value;
    console.log(value)
    setSelectedValue(value);
  };

  const openFilePath = async () => {
    await invoke("open_path", { path: selectedValue })
  };

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
      let saveLocation: PossiblePath[] = await invoke("find_game_save_paths", { gameName, dirsToCheck })
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
        <div>
          <div className="scroll">
            {searchResult.map((item, index) => (
              <div key={index}>
                <label 
                  style={index === 0 ? { fontWeight: 'bold', color: '#55ccff', padding: '5px' } : {}}>
                  <input
                    type="radio"
                    name="radio-checkbox-list"
                    value={item.path}
                    style = {{padding: '1px'}}
                    checked={selectedValue === item.path}
                    defaultChecked={index === 0}
                    onChange={handleRadioChange}
                  />
                  {item.path}
                </label>
              </div>
            ))}
          </div>
          <button className="open-path-button" onClick={openFilePath} >Open Possible Save Path</button>
        </div>
      )}

    </div>
  );
}

export default App;
