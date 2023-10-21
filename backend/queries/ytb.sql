--: ChannelInfo(channel_name?, introduction?, description?)

--! channels_by_tags
SELECT DISTINCT ytb_channel.channel
  FROM tag_ytb_channel
       INNER JOIN tag USING (tag_id)
       INNER JOIN ytb_channel USING (ytb_channel_id)
 WHERE tag.tag_name = ANY(:tags); 

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
