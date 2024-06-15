declare global {
  type Channel = {
    id: string;
    title: string;
    custom_url: string;
    description: string;
    thumbnail: string;
    subscriber_count: string;
    view_count: string;
    video_count: string;
  };
}

export {};
