use crate::net::client::Api;
use crate::ops::{auth, mine, mission, profile};
use crate::support::cfg::loader::Setup;
use crate::support::consts::net::{APP_VERSION, TABLE_RANKS};
use crate::support::error::AppResult;
use crate::support::log::format::{cash, clean_text, gap, lg, ly};

pub async fn execute(setup: &Setup) -> AppResult<()> {
    for (index, account) in setup.accounts.iter().enumerate() {
        if index > 0 {
            gap();
        }

        let proxy = setup.proxy_for(index);
        if let Some(proxy) = proxy {
            ly(&format!("Using proxy {}", proxy.masked()));
        }

        let mut api = Api::open(proxy)?;
        let (_session, reused) = auth::session::establish(&setup.root, &mut api, account).await?;

        if reused {
            lg("The stored session was renewed without the password");
        } else {
            lg("The account password was accepted by the server");
        }

        let profile = profile::fetch::perform(&api).await?;
        lg(&format!(
            "Signed in to the account as {} with {}",
            clean_text(&profile.username, &account.masked()),
            cash(profile.coins, "Coins")
        ));
        ly(&format!(
            "The account is at level {} with {} experience and a {} day streak",
            profile.level, profile.xp, profile.streak
        ));
        ly(&format!(
            "The account is ranked {} on the ladder",
            clean_text(&profile.rank, "unranked")
        ));
        ly(&format!(
            "The account holds the referral code {} on the server",
            clean_text(&profile.referral_code, "none")
        ));

        let (missions, progress) = mission::list::perform(&api).await?;
        let ready = missions
            .iter()
            .filter(|item| {
                progress
                    .iter()
                    .any(|row| row.mission_id == item.id && row.completed && !row.claimed)
            })
            .count();
        ly(&format!(
            "The server listed {} missions and {} are ready to claim",
            missions.len(),
            ready
        ));

        ly(&format!("The app build {} is speaking to the server", APP_VERSION));

        match api.read(TABLE_RANKS, "select=id,name,min_coins_required&order=min_coins_required.asc").await {
            Ok(rows) => {
                let count = rows.as_array().map(|list| list.len()).unwrap_or(0);
                ly(&format!("The server listed {} ranks on the ladder", count));
            }
            Err(err) => ly(&format!("The ranks could not be read because {}", err)),
        }

        match mine::session::active(&api).await? {
            Some(session) => ly(&format!(
                "The mining session has {} left at {} per hour",
                mine::session::describe(session.remaining_ms()),
                cash(session.rate, "Coins")
            )),
            None => ly("No mining session is running on the server"),
        }
    }

    Ok(())
}
