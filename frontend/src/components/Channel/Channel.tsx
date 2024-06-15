import "./Channel.css";

function Channel({ channel }: { channel: Channel }) {
  return (
    <>
      <div id="channel">
        <img src={channel.thumbnail} alt="Channel thumbnail." />
        <p>{channel.title}</p>
      </div>
    </>
  );
}

export default Channel;
