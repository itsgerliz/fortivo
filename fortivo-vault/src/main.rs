use libfortivo::error::FortivoResult;

fn main() -> FortivoResult<()> {
	#[cfg(feature = "logging")]
	env_logger::init();
	
	Ok(())
}
