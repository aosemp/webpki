// Copyright 2015-2020 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

mod dns_name;
pub use dns_name::{DnsNameRef, InvalidDnsNameError};

/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub use dns_name::DnsName;

#[allow(clippy::module_inception)]
mod name;
pub use name::{DnsNameOrIpRef, InvalidDnsNameOrIpError};

pub mod ip_address;

mod verify;
pub(super) use verify::{check_name_constraints, verify_cert_dns_name, verify_cert_dns_name_or_ip};
