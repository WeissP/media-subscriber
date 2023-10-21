--! insert_tag
INSERT INTO tag (tag_name) VALUES
(:tag_name)
ON CONFLICT DO NOTHING;

--! tag_id : (tag_id?)
SELECT tag_id
  FROM tag
 WHERE tag_id = :tag_name;
