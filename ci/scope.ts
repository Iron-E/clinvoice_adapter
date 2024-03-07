import { setTo } from '@iron-e/scabbard';
import { CARGO_HACK_VERSION } from '@iron-e/scabbard/rust/scope/client';

setTo('0.6.20', CARGO_HACK_VERSION);

/**
 * Default arguments for cargo hack.
 */
export const CARGO_HACK_ARGS = [
	'--feature-powerset',
	'--at-least-one-of',
	'sqlx-runtime-actix-native-tls,\
sqlx-runtime-actix-rustls,\
sqlx-runtime-async-std-native-tls,\
sqlx-runtime-async-std-rustls,\
sqlx-runtime-tokio-native-tls,\
sqlx-runtime-tokio-rustls',
];
