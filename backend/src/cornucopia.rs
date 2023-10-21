// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod tag
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub struct Optioni32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> Option<i32>,
    mapper : fn(Option<i32>) -> T,
} impl < 'a, C, T : 'a, const N : usize > Optioni32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(Option<i32>) -> R) -> Optioni32Query
    < 'a, C, R, N >
    {
        Optioni32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn insert_tag() -> InsertTagStmt
{ InsertTagStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO tag (tag_name) VALUES
($1)
ON CONFLICT DO NOTHING")) } pub
struct InsertTagStmt(cornucopia_async :: private :: Stmt) ; impl
InsertTagStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
tag_name : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [tag_name,]) .await
} }pub fn tag_id() -> TagIdStmt
{ TagIdStmt(cornucopia_async :: private :: Stmt :: new("SELECT tag_id
  FROM tag
 WHERE tag_id = $1")) } pub
struct TagIdStmt(cornucopia_async :: private :: Stmt) ; impl
TagIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
tag_name : & 'a i32,) -> Optioni32Query < 'a, C,
Option<i32>, 1 >
{
    Optioni32Query
    {
        client, params : [tag_name,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }}pub mod ytb
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpdateChannelCacheParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,> { pub channel_name : T1,pub description : T2,pub channel : T3,}#[derive( Debug)] pub struct UpdateChannelIntroParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub introduction : T1,pub channel : T2,}pub struct StringQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> & str,
    mapper : fn(& str) -> T,
} impl < 'a, C, T : 'a, const N : usize > StringQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(& str) -> R) -> StringQuery
    < 'a, C, R, N >
    {
        StringQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq, )] pub struct VideoInfo
{ pub video : String,pub video_title : Option<String>,pub video_length : Option<i32>,pub introduction : Option<String>,pub description : Option<String>,pub published : Option<time::OffsetDateTime>,pub cached_at : Option<time::OffsetDateTime>,pub updated_at : Option<time::OffsetDateTime>,}pub struct VideoInfoBorrowed < 'a >
{ pub video : &'a str,pub video_title : Option<&'a str>,pub video_length : Option<i32>,pub introduction : Option<&'a str>,pub description : Option<&'a str>,pub published : Option<time::OffsetDateTime>,pub cached_at : Option<time::OffsetDateTime>,pub updated_at : Option<time::OffsetDateTime>,} impl < 'a > From < VideoInfoBorrowed <
'a >> for VideoInfo
{
    fn
    from(VideoInfoBorrowed { video,video_title,video_length,introduction,description,published,cached_at,updated_at,} : VideoInfoBorrowed < 'a >)
    -> Self { Self { video: video.into(),video_title: video_title.map(|v| v.into()),video_length,introduction: introduction.map(|v| v.into()),description: description.map(|v| v.into()),published,cached_at,updated_at,} }
}pub struct VideoInfoQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> VideoInfoBorrowed,
    mapper : fn(VideoInfoBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > VideoInfoQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(VideoInfoBorrowed) -> R) -> VideoInfoQuery
    < 'a, C, R, N >
    {
        VideoInfoQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq, )] pub struct ChannelInfo
{ pub channel : String,pub channel_name : Option<String>,pub introduction : Option<String>,pub description : Option<String>,}pub struct ChannelInfoBorrowed < 'a >
{ pub channel : &'a str,pub channel_name : Option<&'a str>,pub introduction : Option<&'a str>,pub description : Option<&'a str>,} impl < 'a > From < ChannelInfoBorrowed <
'a >> for ChannelInfo
{
    fn
    from(ChannelInfoBorrowed { channel,channel_name,introduction,description,} : ChannelInfoBorrowed < 'a >)
    -> Self { Self { channel: channel.into(),channel_name: channel_name.map(|v| v.into()),introduction: introduction.map(|v| v.into()),description: description.map(|v| v.into()),} }
}pub struct ChannelInfoQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> ChannelInfoBorrowed,
    mapper : fn(ChannelInfoBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > ChannelInfoQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(ChannelInfoBorrowed) -> R) -> ChannelInfoQuery
    < 'a, C, R, N >
    {
        ChannelInfoQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn channels_by_tag() -> ChannelsByTagStmt
{ ChannelsByTagStmt(cornucopia_async :: private :: Stmt :: new("SELECT DISTINCT ytb_channel.channel
  FROM tag_ytb_channel
       INNER JOIN tag USING (tag_id)
       INNER JOIN ytb_channel USING (ytb_channel_id)
 WHERE tag.tag_name = $1")) } pub
struct ChannelsByTagStmt(cornucopia_async :: private :: Stmt) ; impl
ChannelsByTagStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
tag : & 'a T1,) -> StringQuery < 'a, C,
String, 1 >
{
    StringQuery
    {
        client, params : [tag,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it.into() },
    }
} }pub fn videos_by_tag() -> VideosByTagStmt
{ VideosByTagStmt(cornucopia_async :: private :: Stmt :: new("SELECT DISTINCT
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
 WHERE tag.tag_name = $1")) } pub
struct VideosByTagStmt(cornucopia_async :: private :: Stmt) ; impl
VideosByTagStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
tag : & 'a T1,) -> VideoInfoQuery < 'a, C,
VideoInfo, 1 >
{
    VideoInfoQuery
    {
        client, params : [tag,], stmt : & mut self.0, extractor :
        | row | { VideoInfoBorrowed { video : row.get(0),video_title : row.get(1),video_length : row.get(2),introduction : row.get(3),description : row.get(4),published : row.get(5),cached_at : row.get(6),updated_at : row.get(7),} }, mapper : | it | { <VideoInfo>::from(it) },
    }
} }pub fn channel_info() -> ChannelInfoStmt
{ ChannelInfoStmt(cornucopia_async :: private :: Stmt :: new("SELECT
  channel,
  channel_name,
  introduction,
  description
  FROM ytb_channel
 WHERE channel = $1")) } pub
struct ChannelInfoStmt(cornucopia_async :: private :: Stmt) ; impl
ChannelInfoStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
channel : & 'a T1,) -> ChannelInfoQuery < 'a, C,
ChannelInfo, 1 >
{
    ChannelInfoQuery
    {
        client, params : [channel,], stmt : & mut self.0, extractor :
        | row | { ChannelInfoBorrowed { channel : row.get(0),channel_name : row.get(1),introduction : row.get(2),description : row.get(3),} }, mapper : | it | { <ChannelInfo>::from(it) },
    }
} }pub fn insert_channel() -> InsertChannelStmt
{ InsertChannelStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO ytb_channel (channel)
VALUES ($1) ON CONFLICT (channel) DO NOTHING")) } pub
struct InsertChannelStmt(cornucopia_async :: private :: Stmt) ; impl
InsertChannelStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
channel : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [channel,]) .await
} }pub fn update_channel_cache() -> UpdateChannelCacheStmt
{ UpdateChannelCacheStmt(cornucopia_async :: private :: Stmt :: new("UPDATE ytb_channel
   SET
       channel_name = $1,
       description  = $2,
       cached_at    = NOW()       
 WHERE channel = $3")) } pub
struct UpdateChannelCacheStmt(cornucopia_async :: private :: Stmt) ; impl
UpdateChannelCacheStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
channel_name : & 'a T1,description : & 'a T2,channel : & 'a T3,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [channel_name,description,channel,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UpdateChannelCacheParams < T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UpdateChannelCacheStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpdateChannelCacheParams < T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.channel_name,& params.description,& params.channel,) ) }
}pub fn update_channel_intro() -> UpdateChannelIntroStmt
{ UpdateChannelIntroStmt(cornucopia_async :: private :: Stmt :: new("UPDATE ytb_channel
   SET introduction = $1
 WHERE channel = $2")) } pub
struct UpdateChannelIntroStmt(cornucopia_async :: private :: Stmt) ; impl
UpdateChannelIntroStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
introduction : & 'a T1,channel : & 'a T2,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [introduction,channel,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UpdateChannelIntroParams < T1,T2,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UpdateChannelIntroStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpdateChannelIntroParams < T1,T2,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.introduction,& params.channel,) ) }
}}}