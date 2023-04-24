%%%
Title = "An RDAP Extensions Media Type"
area = "Internet"
workgroup = "Network Working Group"

[seriesInfo]
name = "Internet-Draft"
value = "draft-newton-regext-ext-json-media-type-00"
stream = "IETF"
status = "standards track"

date = 2023-04-21T00:00:00Z

[[author]]
initials="A."
surname="Newton"
fullname="Andy Newton"
organization="ICANN"

[[author]]
initials="J."
surname="Singh"
fullname="Jasdip Singh"
organization="ARIN"
%%%

.# Abstract

This document defines a media type for RDAP that can be used to describe RDAP content
with RDAP extensions. Additionally, this document describes the usage of this media
type with RDAP.

{mainmatter}

# First Section

RFC 7480 defines the 'application/rdap+json' media type to be used with RDAP. This
document defines a new media type to be used in conjuction with the current media type
when an RDAP extension needs to be described during HTTP content negotiation.

# The RDAP With Extensions Media Type

The media type defined by this document is 'application/rdapx+json'. This media
type has a parameter of "extensions" which is a whitespace separated list of RDAP
extensions as defined in the IANA RDAP Extensions registry.

Here is an example:

    application/rdapx+json;extensions="rdap_level_0 fred"
    
# Using The RDAP With Extensions Media Type

RFC 7480 specifies the usage of 'application/json', 'application/rdap+json' or
both with HTTP Accept header. When using the media type defined by this document,
the 'application/rdap+json' media type MUST also be used in the Accept header.

An example:

    accept: application/rdap+json, application/rdapx+json;extensions="rdap_level_0 fred"
    
When a server is programmed to understand the RDAP With Extensions media type,
it should respond with this media type in the Content-Type header. By doing so,
clients will be able to detect if the server recognizes the media type. Otherwise,
the server will use the 'application/rdap+json' media type signalling to the client
that the RDAP With Extensions media type is not recognized by the server.

When the RDAP With Extensions media type is used in the Content-Type header, the
values in the media type's extension parameter MUST match the values in the `rdapConformance`
array in the return JSON.

# IANA Considerations

Type name: application

Subtype name: rdapx+json

Required parameters: This media type has a parameter of "extensions" which is a whitespace-separated list of RDAP extensions as defined in the IANA RDAP Extensions registry.

Optional parameters:  N/A

Encoding considerations: See Section 3.1 of [RFC6839].

Security considerations: The media represented by this identifier
does not have security considerations beyond that found in
Section 12 of [RFC8259].

Interoperability considerations: There are no known
interoperability problems regarding this media format.

Published specification: This document.

Applications that use this media type: Implementations of the
Registration Data Access Protocol (RDAP) with Extensions.

Additional information: This media type is a product of the IETF
REGEXT Working Group.  The REGEXT charter, information on the
REGEXT mailing list, and other documents produced by the REGEXT
Working Group can be found at https://datatracker.ietf.org/wg/
regext/.

Person & email address to contact for further information: IESG
<iesg&ietf.org>

Intended usage: COMMON

Restrictions on usage: none

Author: Andy Newton

Change controller: IETF

Provisional Registration: No

{backmatter}

# Design Considerations

## Not Reusing the Existing Media Type

Section 4.3 of RFC 6838 strongly discourages the creation of new parameters on existing
media types to enable new features. As RDAP has always had extensions, it could be argued
that adding an "extensions" parameter to the existing "application/rdap+json" media type
is not adding a new feature to RDAP. However, the opposite could be argued that adding
the capability for clients to signal desired RDAP extensions is a new feature.

More practically, there is concern that adding a new parameter to the existing media
type would not be backward compatible with some server software. That is, servers
examining media types as exact string matches may incorrectly conclude that the existing
media type with an unknown, new parameter may not be the same as the existing media
type without parameters. A similar, though less likely, concern exists for clients.

As servers are required to handle multiple media types according to RFC 7480 and RFC 9110,
it therefore seems reasonable to conclude that defining a new media type for use with
the existing media type is best to preserve backward compatibility.

## Query Parameters Considered Harmful

Another design approach to communicating RDAP extensions from the client to the
server would be the use of URI query parameters:

```
https://rdap.example/domain/foo.example?extensions=fizz%20buzz  
```

### Copy and Paste

Consider two RDAP users, Alice and Bob. Alice has an RDAP client that supports
the extensions "fizzbuzz", and Bob has an RDAP client that does not support this
extension.

Now consider the scenario where Alice copies and pastes the RDAP URL from above into an email
and sends it to Bob. When Bob uses that URL with his RDAP client, it will be communicating
to the server that the extension "fizzbuzz" is understood by Bob's client when it is not.

In this scenario, Bob's client will be unable to render the RDAP extension regardless
of the usage or not of the query parameter. However, if the server is using the query
parameter for secondary purposes, such as gathering metrics and statistics, then the
capabilities of Bob's client will have been incorrectly signalled to the server.

### Redirects

The RDAP ecosystem uses redirects in many situations. RFC 7480 discusses "aggregators", which
are RDAP servers used to help clients find authoritative RDAP servers using the RDAP bootstrap
registires. Redirects are also heavily used by the RIRs when IP addresses or autonomous
system numbers are transferred from one RIR to another.

Within HTTP, URI query parameters are not explicitly preserved during a redirect (probably
due to architecture considerations, see the section below). Specific to RDAP, RFC 7480
instructs RDAP servers to ignore unknown query parameters and instructs clients not to
transform a URL of a redirect.

Therefore, query parameters denoting RDAP extentions will not survive redirects. This can
be readily observed in currently deployed RDAP servers:

```
curl -v https://rdap-bootstrap.arin.net/bootstrap/autnum/2830?extension=fizzbuzz    
```

To further demonstrate that query parameters do not survive redirects but that media types
do survive redirects, considered the code found [here](https://github.com/anewton1998/draft-regext-ext-json-media-type).
This code consists of a simple client and a simple server. The client sets both a new
media type and query parameters. The servers listens on two ports, redirecting the client
from a URL on the first port to a URL on the second port.

Here is the output of the client. It shows that the query parameters are not automatically
preserved but that the media type is automatically preserved.

```
2023-04-23T20:08:34.630798Z  INFO client: sending reqwest
2023-04-23T20:08:34.673776Z DEBUG reqwest::connect: starting new connection: http://127.0.0.1:3000/
2023-04-23T20:08:34.673885Z DEBUG hyper::client::connect::http: connecting to 127.0.0.1:3000
2023-04-23T20:08:34.674118Z DEBUG hyper::client::connect::http: connected to 127.0.0.1:3000
2023-04-23T20:08:34.675110Z DEBUG hyper::proto::h1::io: flushed 148 bytes
2023-04-23T20:08:34.675995Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-04-23T20:08:34.676025Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-04-23T20:08:34.676120Z DEBUG hyper::client::pool: pooling idle connection for ("http", 127.0.0.1:3000)
2023-04-23T20:08:34.676237Z DEBUG reqwest::async_impl::client: redirecting 'http://127.0.0.1:3000/ex1/domain/foo.example?foo=bar' to 'http://127.0.0.1:4000/ex2/domain/foo.example'
2023-04-23T20:08:34.676311Z DEBUG reqwest::connect: starting new connection: http://127.0.0.1:4000/
2023-04-23T20:08:34.676351Z DEBUG hyper::client::connect::http: connecting to 127.0.0.1:4000
2023-04-23T20:08:34.676547Z DEBUG hyper::client::connect::http: connected to 127.0.0.1:4000
2023-04-23T20:08:34.676987Z DEBUG hyper::proto::h1::io: flushed 203 bytes
2023-04-23T20:08:34.677640Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-04-23T20:08:34.677680Z DEBUG hyper::proto::h1::conn: incoming body is content-length (66 bytes)
2023-04-23T20:08:34.677747Z DEBUG hyper::proto::h1::conn: incoming body completed
2023-04-23T20:08:34.677829Z DEBUG hyper::client::pool: pooling idle connection for ("http", 127.0.0.1:4000)
2023-04-23T20:08:34.677850Z  INFO client: returned content type: "application/extrdap;extensions=\"foo bar\""
2023-04-23T20:08:34.677883Z  INFO client: status code is 418 I'm a teapot
2023-04-23T20:08:34.677996Z  INFO client: response is {"errorCode":418,"title": "Your Beverage Choice is Not Available"}    
```

Here is the output of the server. It show that the client, upon redirect, automatically sends the media type
but does not automatically preserve the query parameters.

```
2023-04-23T20:08:22.055030Z  INFO servers: starting server on port 4000
2023-04-23T20:08:22.055036Z  INFO servers: starting server on port 3000
2023-04-23T20:08:34.675533Z DEBUG hyper::proto::h1::io: parsed 2 headers
2023-04-23T20:08:34.675564Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-04-23T20:08:34.675661Z  INFO servers: Serving request from 127.0.0.1:41776
2023-04-23T20:08:34.675680Z  INFO servers: accept values: "application/rdap+json, application/extrdap+json;extensions=\"foo bar\""
2023-04-23T20:08:34.675698Z  INFO servers: redirecting to server on port 4000
2023-04-23T20:08:34.675804Z DEBUG hyper::proto::h1::io: flushed 147 bytes
2023-04-23T20:08:34.677098Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-04-23T20:08:34.677137Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-04-23T20:08:34.677276Z  INFO servers: Serving request from 127.0.0.1:41930
2023-04-23T20:08:34.677302Z  INFO servers: accept values: "application/rdap+json, application/extrdap+json;extensions=\"foo bar\""
2023-04-23T20:08:34.677342Z  INFO servers: responding with an unuseful error
2023-04-23T20:08:34.677481Z DEBUG hyper::proto::h1::io: flushed 208 bytes
2023-04-23T20:08:34.678847Z DEBUG hyper::proto::h1::conn: read eof
2023-04-23T20:08:34.679031Z DEBUG hyper::proto::h1::conn: read eof    
```

Preservation of query parameters is not a common feature of HTTP client and server libraries,
whereas preservation of media types is common.

### Architectual Violations

As noted in RFC 3986, URI query parameters are meant to be part of the identity of the resource
being identified by a URI and pointed to by the location of a URL. RDAP extensions change
the portions of JSON returned by the server but are not intended to change the resource
being identified. That is, a domain registration is the same domain registration regardless
of whether the postal address in that domain registration is communicated via JCard or
a new RDAP extensions for JSContact.

Changing how the content of a resource is conveyed is called content negotiation and
is discussed in detail in RFC 9110 using media types.

Readers should note that protocol design is not a "priestly affair" in which architectural
violations are strictly forbidden. Every design decision is a trade-off. However, following
the architecture of an ecosystem generally makes re-use of software and systems easier,
and often eases the adoption of newer features in the future. When given the choice between
two designs, the design that does not violate architecture should be preferred. 

## RDAP Extension Versioning

It is beyond the scope of this document to define the versioning of RDAP extensions.
However, there is design intent to allow the use of explicitly versioned RDAP extension
identifiers where they are also compatible with the identifiers used in the `rdapConformance`
array of RDAP.

Consider the scenario in which the IETF decides that RDAP extension identifiers suffixed with
the character string `__V` denotes RDAP extensions versioned using a semantic versioning
scheme. In this scenario, the RDAP extension identifier `fizzbuzz__V` is registered with IANA.
The `__V` suffix indicates that when the identifier is used in the `rdapConformance` array,
it must appear appended with a character string denoting the semantic version of the extension.

For example, `fizzbuzz__V_2_1` denotes version 2.1 of the fizzbuzz extension. In RDAP JSON,
the conformance would appear as:

```
"rdapConformance" : [
    "rdap_level_0", 
    "fizzbuzz__V_2_1" 
]    
```

The ussage with the `rdax` media type would be:

```
application/rdapx+json;extensions="rdap_level_0 fizzbuzz__V_2_1"    
```

Readers should note that this scenario is provided to show design intent and is not
a full-fledged extension versioning design. Additionally, the new media type
defined in this document has utility with existing, opaquely versioned RDAP extensions
and does not depend on the definition of a new versioning scheme for RDAP extensions.
