// This file is part of Substrate.

// Copyright (C) 2021-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use sc_utils::notification::{NotificationSender, NotificationStream, TracingKeyStr};
use sp_runtime::traits::Block as BlockT;

use crate::justification::BeefySignedCommitment;

/// The sending half of the notifications channel(s) used to send
/// notifications about best BEEFY block from the gadget side.
pub type BeefyBestBlockSender<Block> = NotificationSender<<Block as BlockT>::Hash>;

/// The receiving half of a notifications channel used to receive
/// notifications about best BEEFY blocks determined on the gadget side.
pub type BeefyBestBlockStream<Block> =
	NotificationStream<<Block as BlockT>::Hash, BeefyBestBlockTracingKey>;

/// The sending half of the notifications channel(s) used to send notifications
/// about signed commitments generated at the end of a BEEFY round.
pub type BeefySignedCommitmentSender<Block> = NotificationSender<BeefySignedCommitment<Block>>;

/// The receiving half of a notifications channel used to receive notifications
/// about signed commitments generated at the end of a BEEFY round.
pub type BeefySignedCommitmentStream<Block> =
	NotificationStream<BeefySignedCommitment<Block>, BeefySignedCommitmentTracingKey>;

/// Provides tracing key for BEEFY best block stream.
#[derive(Clone)]
pub struct BeefyBestBlockTracingKey;
impl TracingKeyStr for BeefyBestBlockTracingKey {
	const TRACING_KEY: &'static str = "mpsc_beefy_best_block_notification_stream";
}

/// Provides tracing key for BEEFY signed commitments stream.
#[derive(Clone)]
pub struct BeefySignedCommitmentTracingKey;
impl TracingKeyStr for BeefySignedCommitmentTracingKey {
	const TRACING_KEY: &'static str = "mpsc_beefy_signed_commitments_notification_stream";
}
