use anyhow::{anyhow, Context, Result};

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

#[allow(unused_imports)]
use ctap_hid_fido2::util;
use ctap_hid_fido2::{HidParam, InfoOption};

use ctap_hid_fido2::credential_management_params::Credential;
use ctap_hid_fido2::credential_management_params::Rp;
use ctap_hid_fido2::public_key_credential_user_entity::PublicKeyCredentialUserEntity;
use ctap_hid_fido2::verifier;

pub fn memo(matches: &clap::ArgMatches) -> Result<()> {

    if is_supported()? == false {
        return Err(anyhow!(
            "Sorry , This authenticator is not supported for this functions."
        ));
    }

    let pin = if matches.is_present("pin") {
        matches.value_of("pin").unwrap().to_string()
    } else {
        println!("pin:");

        let input = get_input();
        println!();
        input
    };

    let rpid = "ctapcli";

    if matches.is_present("add") {
        let mut values = matches.values_of("add").context("arg")?;
        let tag = values.next().context("arg")?;
        let memo = values.next().context("arg")?;

        if let None = search_cred(&pin, rpid, tag.as_bytes())? {
            let challenge = verifier::create_challenge();
            let rkparam =
                PublicKeyCredentialUserEntity::new(Some(tag.as_bytes()), Some(memo), None);

            let _att = ctap_hid_fido2::make_credential_rk(
                &HidParam::get_default_params(),
                rpid,
                &challenge,
                Some(&pin),
                &rkparam,
            )?;

            println!("Add Success!.");
        } else {
            println!("This tag already exists...");
        }
    } else if matches.is_present("del") {
        let mut values = matches.values_of("del").unwrap();
        let tag = values.next().unwrap();
        println!("Delete a memo => {}.", tag);

        if let Some(cred) = search_cred(&pin, rpid, tag.as_bytes())? {
            ctap_hid_fido2::credential_management_delete_credential(
                &HidParam::get_default_params(),
                Some(&pin),
                Some(cred.public_key_credential_descriptor),
            )?;

            println!("Delete Success!.");
        } else {
            println!("tag not found...");
        }
    } else if matches.is_present("get") {
        let mut values = matches.values_of("get").unwrap();
        let tag = values.next().unwrap();
        get(tag, &pin, rpid)?;
    } else if matches.is_present("list") {
        println!("List All Memos.");
        list(&pin,rpid)?;
    } else {
        list(&pin,rpid)?;

        println!();
        println!("tag:");
        let tag = get_input();

        get(&tag, &pin, rpid)?;
    }

    Ok(())
}

fn is_supported() -> Result<bool> {
    if let None = ctap_hid_fido2::enable_info_option(
        &HidParam::get_default_params(),
        &InfoOption::CredentialMgmtPreview,
    )? {
        if let None = ctap_hid_fido2::enable_info_option(
            &HidParam::get_default_params(),
            &InfoOption::CredMgmt,
        )? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn get_rps(pin: Option<&str>) -> Result<Vec<Rp>> {
    ctap_hid_fido2::credential_management_enumerate_rps(&HidParam::get_default_params(), pin)
}

fn get_creds(pin: Option<&str>, rp: &Rp) -> Result<Vec<Credential>> {
    ctap_hid_fido2::credential_management_enumerate_credentials(
        &HidParam::get_default_params(),
        pin,
        &rp.rpid_hash,
    )
}

fn search_cred(pin: &str, rpid: &str, user_entity_id: &[u8]) -> Result<Option<Credential>> {
    let rps = get_rps(Some(pin))?;

    let mut rps = rps
        .iter()
        .filter(|it| it.public_key_credential_rp_entity.id == rpid);

    if let Some(r) = rps.next() {
        let creds = get_creds(Some(pin), r)?;

        let mut creds = creds
            .iter()
            .filter(|it| it.public_key_credential_user_entity.id.eq(user_entity_id));

        if let Some(c) = creds.next() {
            return Ok(Some(c.clone()));
        }
    }
    Ok(None)
}

fn get(tag: &str, pin: &str, rpid: &str) -> Result<()> {
    println!("Get a memo => {}.", tag);

    if let Some(cred) = search_cred(pin, rpid, tag.as_bytes())? {
        //let tag = String::from_utf8(cred.public_key_credential_user_entity.id)?;
        //println!("- tag = {}", tag);

        let data = cred.public_key_credential_user_entity.name;
        //println!("- {}", data);

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(data.to_owned()).unwrap();

        println!("Copied it to the clipboard :) :) :) !");
    } else {
        println!("tag not found...");
    }
    Ok(())
}

fn list(pin: &str, rpid: &str) -> Result<()> {
    let rps = get_rps(Some(pin))?;
    let mut rps = rps
        .iter()
        .filter(|it| it.public_key_credential_rp_entity.id == rpid);

    if let Some(r) = rps.next() {
        let creds = get_creds(Some(&pin), r)?;

        for id in creds
            .iter()
            .map(|it| it.public_key_credential_user_entity.id.to_vec())
        {
            let tag = String::from_utf8(id)?;
            println!("- {}", tag);
        }

        println!("({}/10)", creds.len());
    }
    Ok(())
}

fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}
