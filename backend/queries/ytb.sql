--: ChannelInfo(channel_name?, introduction?, description?)
--: VideoInfo(video_title?,video_length?,introduction?,description?,published?,cached_at?,updated_at?)

--! channels_by_tag
SELECT DISTINCT ytb_channel.channel
  FROM tag_ytb_channel
       INNER JOIN tag USING (tag_id)
       INNER JOIN ytb_channel USING (ytb_channel_id)
 WHERE tag.tag_name = :tag; 

--! videos_by_tag : VideoInfo
SELECT DISTINCT
  ytb_video.video,
  ytb_video.video_title,
  ytb_video.video_length,
  ytb_video.introduction,
  ytb_video.description,
  ytb_video.published,
  ytb_video.cached_at,
  ytb_video.updated_at
  FROM tag_ytb_video
       INNER JOIN tag USING (tag_id)
       INNER JOIN ytb_video USING (ytb_video_id)
 WHERE tag.tag_name = :tag; 


--! channel_info : ChannelInfo
SELECT
  channel,
  channel_name,
  introduction,
  description
  FROM ytb_channel
 WHERE channel = :channel;

--! insert_channel
INSERT INTO ytb_channel (channel)
VALUES (:channel) ON CONFLICT (channel) DO NOTHING;

--! update_channel_cache
UPDATE ytb_channel
   SET
       channel_name = :channel_name,
       description  = :description,
       cached_at    = NOW()       
 WHERE channel = :channel;


--! update_channel_intro
UPDATE ytb_channel
   SET introduction = :introduction
 WHERE channel = :channel;
