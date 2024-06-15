import { useState } from "react";
import Channel from "../Channel/Channel.tsx";
import "./Search.css";

function Search() {
  const [results, setResults] = useState([]);
  const [query, setQuery] = useState("");

  async function getResults() {
    try {
      const response = await fetch(`http://127.0.0.1:8001/search?query=${query}`);
      const data = await response.json();
      setResults(data);
    } catch (err) {}
  }

  return (
    <>
      <div id="search">
        <input
          id="searchBar"
          type="text"
          placeholder="Search for YouTube channels..."
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            getResults();
          }}
        />
        <div id="results">
          {results.map((channel, i) => (
            <>
              <Channel key={i} channel={channel}></Channel>
            </>
          ))}
        </div>
      </div>
    </>
  );
}

export default Search;
