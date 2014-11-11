use serde::ser;
use serde::de;

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Http {
    protocol: HttpProtocol,
    status: u32,
    host_status: u32,
    up_status: u32,
    method: HttpMethod,
    content_type: String,
    user_agent: String,
    referer: String,
    request_uri: String,
}

#[allow(non_camel_case_types)]
#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum HttpProtocol {
    HTTP_PROTOCOL_UNKNOWN,
    HTTP10,
    HTTP11,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for HttpProtocol {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for HttpProtocol {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<HttpProtocol, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum HttpMethod {
    METHOD_UNKNOWN,
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    PURGE,
    OPTIONS,
    PROPFIND,
    MKCOL,
    PATCH,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for HttpMethod {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for HttpMethod {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<HttpMethod, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum CacheStatus {
    CACHESTATUS_UNKNOWN,
    Miss,
    Expired,
    Hit,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for CacheStatus {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for CacheStatus {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<CacheStatus, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Origin {
    ip: String,
    port: u32,
    hostname: String,
    protocol: OriginProtocol,
}

#[allow(non_camel_case_types)]
#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum OriginProtocol {
    ORIGIN_PROTOCOL_UNKNOWN,
    HTTP,
    HTTPS,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for OriginProtocol {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for OriginProtocol {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<OriginProtocol, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum ZonePlan {
    ZONEPLAN_UNKNOWN,
    FREE,
    PRO,
    BIZ,
    ENT,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for ZonePlan {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for ZonePlan {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<ZonePlan, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Show, Encodable, Decodable, FromPrimitive)]
pub enum Country {
	UNKNOWN,
	A1,
	A2,
	O1,
	AD,
	AE,
	AF,
	AG,
	AI,
	AL,
	AM,
	AO,
	AP,
	AQ,
	AR,
	AS,
	AT,
	AU,
	AW,
	AX,
	AZ,
	BA,
	BB,
	BD,
	BE,
	BF,
	BG,
	BH,
	BI,
	BJ,
	BL,
	BM,
	BN,
	BO,
	BQ,
	BR,
	BS,
	BT,
	BV,
	BW,
	BY,
	BZ,
	CA,
	CC,
	CD,
	CF,
	CG,
	CH,
	CI,
	CK,
	CL,
	CM,
	CN,
	CO,
	CR,
	CU,
	CV,
	CW,
	CX,
	CY,
	CZ,
	DE,
	DJ,
	DK,
	DM,
	DO,
	DZ,
	EC,
	EE,
	EG,
	EH,
	ER,
	ES,
	ET,
	EU,
	FI,
	FJ,
	FK,
	FM,
	FO,
	FR,
	GA,
	GB,
	GD,
	GE,
	GF,
	GG,
	GH,
	GI,
	GL,
	GM,
	GN,
	GP,
	GQ,
	GR,
	GS,
	GT,
	GU,
	GW,
	GY,
	HK,
	HM,
	HN,
	HR,
	HT,
	HU,
	ID,
	IE,
	IL,
	IM,
	IN,
	IO,
	IQ,
	IR,
	IS,
	IT,
	JE,
	JM,
	JO,
	JP,
	KE,
	KG,
	KH,
	KI,
	KM,
	KN,
	KP,
	KR,
	KW,
	KY,
	KZ,
	LA,
	LB,
	LC,
	LI,
	LK,
	LR,
	LS,
	LT,
	LU,
	LV,
	LY,
	MA,
	MC,
	MD,
	ME,
	MF,
	MG,
	MH,
	MK,
	ML,
	MM,
	MN,
	MO,
	MP,
	MQ,
	MR,
	MS,
	MT,
	MU,
	MV,
	MW,
	MX,
	MY,
	MZ,
	NA,
	NC,
	NE,
	NF,
	NG,
	NI,
	NL,
	NO,
	NP,
	NR,
	NU,
	NZ,
	OM,
	PA,
	PE,
	PF,
	PG,
	PH,
	PK,
	PL,
	PM,
	PN,
	PR,
	PS,
	PT,
	PW,
	PY,
	QA,
	RE,
	RO,
	RS,
	RU,
	RW,
	SA,
	SB,
	SC,
	SD,
	SE,
	SG,
	SH,
	SI,
	SJ,
	SK,
	SL,
	SM,
	SN,
	SO,
	SR,
	SS,
	ST,
	SV,
	SX,
	SY,
	SZ,
	TC,
	TD,
	TF,
	TG,
	TH,
	TJ,
	TK,
	TL,
	TM,
	TN,
	TO,
	TR,
	TT,
	TV,
	TW,
	TZ,
	UA,
	UG,
	UM,
	US,
	UY,
	UZ,
	VA,
	VC,
	VE,
	VG,
	VI,
	VN,
	VU,
	WF,
	WS,
	XX,
	YE,
	YT,
	ZA,
	ZM,
	ZW,
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for Country {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for Country {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<Country, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Log {
    timestamp: i64,
    zone_id: u32,
    zone_plan: ZonePlan,
    http: Http,
    origin: Origin,
    country: Country,
    cache_status: CacheStatus,
    server_ip: String,
    server_name: String,
    remote_ip: String,
    bytes_dlv: u64,
    ray_id: String,
}

impl Log {
    pub fn new() -> Log {
        Log {
            timestamp: 2837513946597,
            zone_id: 123456,
            zone_plan: FREE,
            http: Http {
                protocol: HTTP11,
                status: 200,
                host_status: 503,
                up_status: 520,
                method: GET,
                content_type: "text/html".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".to_string(),
                referer: "https://www.cloudflare.com/".to_string(),
                request_uri: "/cdn-cgi/trace".to_string(),
            },
            origin: Origin {
                ip: "1.2.3.4".to_string(),
                port: 8000,
                hostname: "www.example.com".to_string(),
                protocol: HTTPS,
            },
            country: US,
            cache_status: Hit,
            server_ip: "192.168.1.1".to_string(),
            server_name: "metal.cloudflare.com".to_string(),
            remote_ip: "10.1.2.3".to_string(),
            bytes_dlv: 123456,
            ray_id: "10c73629cce30078-LAX".to_string(),
        }
    }
}

#[cfg(test)]
mod capnp {
    use std::io::{MemWriter, BufReader};
    use test::Bencher;

    use capnp;
    use capnp::message::MallocMessageBuilder;
    use capnp::ReaderOptions;
    use capnp::message::{MessageReader, MessageBuilder};

    use log_capnp;
    use country_capnp;

    fn new_capnp_log<'a, M: MessageBuilder<'a>>(msg: &'a mut M) -> log_capnp::log::Builder<'a> {
        let log = msg.init_root::<log_capnp::log::Builder>();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(log_capnp::zone_plan::Free);

        let http = log.init_http();
        http.set_protocol(log_capnp::h_t_t_p::protocol::Http11);
        http.set_host_status(200);
        http.set_up_status(520);
        http.set_method(log_capnp::h_t_t_p::method::Get);
        http.set_content_type("text/html");
        http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
        http.set_referer("https://www.cloudflare.com/");
        http.set_request_u_r_i("/cdn-cgi/trace");

        let origin = log.init_origin();
        origin.set_ip("1.2.3.4");
        origin.set_port(8000);
        origin.set_hostname("www.example.com");
        origin.set_protocol(log_capnp::origin::protocol::Https);

        log.set_country(country_capnp::country::Us);
        log.set_cache_status(log_capnp::cache_status::Hit);
        log.set_server_ip("192.168.1.1");
        log.set_server_name("metal.cloudflare.com");
        log.set_remote_ip("10.1.2.3");
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX");

        log
    }

    #[bench]
    fn bench_serialize(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_capnp_log(&mut msg);

        let mut wr = MemWriter::new();
        capnp::serialize::write_message(&mut wr, &msg).unwrap();
        b.bytes = wr.unwrap().len() as u64;

        b.iter(|| {
            let mut wr = MemWriter::new();
            capnp::serialize::write_message(&mut wr, &msg).unwrap();
        });
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_capnp_log(&mut msg);

        let mut wr = MemWriter::new();
        capnp::serialize::write_message(&mut wr, &msg).unwrap();
        let bytes = wr.unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = BufReader::new(bytes.as_slice());
            let message_reader = capnp::serialize::new_reader(&mut rdr, ReaderOptions::new()).unwrap();
            let _: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>();
        });
    }

    #[bench]
    fn bench_serialize_packed_unbuffered(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_capnp_log(&mut msg);
    
        let mut wr = MemWriter::new();
        capnp::serialize_packed::write_packed_message_unbuffered(&mut wr, &msg).unwrap();
        b.bytes = wr.unwrap().len() as u64;

        b.iter(|| {
            let mut wr = MemWriter::new();
            capnp::serialize_packed::write_packed_message_unbuffered(&mut wr, &msg).unwrap();
        });
    }

    #[bench]
    fn bench_deserialize_packed_unbuffered(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_capnp_log(&mut msg);
    
        let mut wr = MemWriter::new();
        capnp::serialize_packed::write_packed_message_unbuffered(&mut wr, &msg).unwrap();
        let bytes = wr.unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = BufReader::new(bytes.as_slice());
            let message_reader = capnp::serialize_packed::new_reader_unbuffered(
                &mut rdr,
                ReaderOptions::new()
            ).unwrap();
            let _: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>();
        });
    }
}

#[cfg(test)]
mod serialize_json {
    use test::Bencher;
    use serialize::json;
    use serialize::Decodable;

    use super::Log;

    static SERIALIZED_JSON: &'static str = r#"{"timestamp":2837513946597,"zone_id":123456,"zone_plan":"FREE","http":{"protocol":"HTTP11","status":200,"host_status":503,"up_status":520,"method":"GET","content_type":"text/html","user_agent":"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML,like Gecko) Chrome/33.0.1750.146 Safari/537.36","referer":"https://www.cloudflare.com/","request_uri":"/cdn-cgi/trace"},"origin":{"ip":"1.2.3.4","port":8000,"hostname":"www.example.com","protocol":"HTTPS"},"country":"US","cache_status":"Hit","server_ip":"192.168.1.1","server_name":"metal.cloudflare.com","remote_ip":"10.1.2.3","bytes_dlv":123456,"ray_id":"10c73629cce30078-LAX"}"#;

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let json = json::encode(&log);
        b.bytes = json.len() as u64;

        b.iter(|| {
            let _ = json::encode(&log);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        b.bytes = SERIALIZED_JSON.len() as u64;

        b.iter(|| {
            let json = json::from_str(SERIALIZED_JSON).unwrap();
            let mut decoder = json::Decoder::new(json);
            let _log: Log = Decodable::decode(&mut decoder).unwrap();
        });
    }
}

#[cfg(test)]
mod serde_json {
    use test::Bencher;
    use serde::json;

    use super::Log;

    static SERIALIZED_JSON: &'static str = r#"{"timestamp":25469139677502,"zone_id":123456,"zone_plan":1,"http":{"protocol":2,"status":200,"host_status":503,"up_status":520,"method":1,"content_type":"text/html","user_agent":"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML,like Gecko) Chrome/33.0.1750.146 Safari/537.36","referer":"https://www.cloudflare.com/","request_uri":"/cdn-cgi/trace"},"origin":{"ip":"1.2.3.4","port":8000,"hostname":"www.example.com","protocol":2},"country":238,"cache_status":3,"server_ip":"192.168.1.1","server_name":"metal.cloudflare.com","remote_ip":"10.1.2.3","bytes_dlv":123456,"ray_id":"10c73629cce30078-LAX"}"#;

    #[bench]
    fn bench_serializer(b: &mut Bencher) {
        let log = Log::new();
        let json = json::to_vec(&log);
        b.bytes = json.len() as u64;

        b.iter(|| {
            let _json = json::to_vec(&log);
        });
    }

    #[bench]
    fn bench_deserializer(b: &mut Bencher) {
        b.bytes = SERIALIZED_JSON.len() as u64;

        b.iter(|| {
            let _log: Log = json::from_str(SERIALIZED_JSON).unwrap();
        });
    }
}

#[cfg(test)]
mod msgpack {
    use std::io::MemWriter;
    use test::Bencher;
    use serialize::Encodable;

    use msgpack;

    use super::Log;

    fn encode(log: &Log) -> Vec<u8> {
        let mut wr = MemWriter::new();
        {
            let mut encoder = msgpack::Encoder::new(&mut wr);
            log.encode(&mut encoder).unwrap();
        }
        wr.unwrap()
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        b.bytes = encode(&log).len() as u64;

        b.iter(|| {
            let _ = encode(&log);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let bytes = encode(&Log::new());
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let _log: Log = msgpack::from_msgpack(bytes.as_slice()).unwrap();
        });
    }
}
