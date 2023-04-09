use actix_web::{Responder, post, HttpResponse};
use serde_json::json;

use crate::handlers::handle_command;


#[post("/api/censor")]
pub async fn censor(
	body: String,
) -> impl Responder {
	let body = format!("/censor {}", body);
	if body.len() == 0 {
		return HttpResponse::InternalServerError().finish();
	} 
	let command: Vec<&str> = body.split_whitespace().collect();
	let cmd = command[0];
	let args = &command[1..].to_vec();
	let res = handle_command(cmd, args.clone());
	
	HttpResponse::Ok().json(json!({ "censored": res }))
}

