import { useEffect, useState } from "react";
import Channel from "../Channel/Channel.tsx";
import "./Search.css";

function Search() {
  const [results, setResults] = useState<Channel[]>([]);
  const [query, setQuery] = useState("");

  async function getResults(query: string) {
    try {
      const response = await fetch(`http://127.0.0.1:8001/search?query=${query}`);
      const data = await response.json();
      const channel = data[0];
      if (results.find((x) => x.id == channel.id)) return;
      const newResults = [channel, ...results];
      if (newResults.length > 5) newResults.splice(5);
      setResults(newResults);
    } catch (err) {}
  }

  useEffect(() => {
    getResults(query);
  }, [query]);

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
          }}
        />
        <div id="results">
          {results.map((channel) => (
            <>
              <Channel key={channel.id} channel={channel}></Channel>
            </>
          ))}
        </div>
      </div>
    </>
  );
}

export default Search;
