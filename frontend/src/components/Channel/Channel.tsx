import { useState } from "react";
import "./Channel.css";

function Channel({ channel }: { channel: Channel }) {
  const subscribers = new Intl.NumberFormat(undefined, {
    notation: "compact",
    compactDisplay: "short",
  }).format(parseInt(channel.subscriber_count));
  const videos = new Intl.NumberFormat(undefined, {
    notation: "compact",
    compactDisplay: "short",
  }).format(parseInt(channel.video_count));

  let [isSubscribed, setIsSubscribed] = useState(false);

  function handleSubscribe() {
    if (isSubscribed) {
      setIsSubscribed(false);
    } else {
      setIsSubscribed(true);
    }
  }

  return (
    <>
      <div id="channel">
        <img src={channel.thumbnail} alt="Channel thumbnail." />
        <div id="text">
          <p id="title">{channel.title}</p>
          <p id="description">
            {channel.description.length > 40
              ? channel.description.slice(0, 40) + "..."
              : channel.description}
          </p>
          <p id="statistics">{`${subscribers} subscribers • ${videos} videos`}</p>
        </div>
        <button
          id="subscribe"
          className={isSubscribed ? "subscribed" : "not-subscribed"}
          onClick={handleSubscribe}
        >
          {isSubscribed ? "Subscribed" : "Subscribe"}
        </button>
      </div>
    </>
  );
}

export default Channel;
