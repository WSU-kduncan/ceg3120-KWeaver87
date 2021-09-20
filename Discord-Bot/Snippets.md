# Snippets of discovered info

## Getting avatar url

Webhook avatars are stored on the CDN by the webhook's id and a hash of the avatar. Example below.

- hook id: 889302278404726784
- avatar hash: "bc35f5665478f03477b5dcd1d538c278"
- UI URL: https://cdn.discordapp.com/avatars/889302278404726784/bc35f5665478f03477b5dcd1d538c278.webp?size=256

```rs
macro_rules! avatar_url {
    () => {
        "https://cdn.discordapp.com/avatars/{}/{}.webp"
    };
}

let avatar_url =
	format!(
		avatar_url!(),
		webhook.id,
		webhook.avatar.expect("Expected webhook avatar"),
	);

```
