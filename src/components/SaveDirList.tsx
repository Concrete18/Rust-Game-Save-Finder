import React from "react";

export interface PossibleDir {
  path: string,
  score: number
}

export interface SaveDirListProps {
  searchResult: PossibleDir[];
  selectedDir: string;
  handleRadioChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const SaveDirList: React.FC<SaveDirListProps> = ({ searchResult, selectedDir, handleRadioChange }) => {
  return (
    <div className="path-container scroll">
      {searchResult.length === 0 ? (
        <div className="no-saves-desc">
          <label className="radio-label">
            <input
              type="radio"
              name="radio-checkbox-list"
              value={12}
              disabled
              checked
              style={{ padding: '1px' }}
              onChange={handleRadioChange} 
            />
            Try searching for a game's save path above.
          </label>
        </div>
      ) : (
        <>
          {searchResult.map((item, index) => (
            <div key={index} className="path-item">
              <label 
                style={index === 0 ? { color: '#55ccff' } : {}} 
                className="radio-label"
              >
                <input
                  type="radio"
                  name="radio-checkbox-list"
                  value={item.path}
                  checked={selectedDir === item.path}
                  onChange={handleRadioChange}
                />
                {item.path}
              </label>
            </div>
          ))}
        </>
      )}
    </div>
  );
};
