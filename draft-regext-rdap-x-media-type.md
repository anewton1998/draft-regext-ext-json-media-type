%%%
Title = "An RDAP With Extensions Media Type"
area = "Applications and Real-Time Area (ART)"
workgroup = "Registration Protocols Extensions (regext)"
abbrev = "rdap-x"
updates = [7480]
ipr= "trust200902"

[seriesInfo]
name = "Internet-Draft"
value = "draft-newton-regext-rdap-x-media-type-00"
stream = "IETF"
status = "standard"
date = 2023-05-16T00:00:00Z

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

[@!RFC7480] defines the 'application/rdap+json' media type to be used with RDAP. This
document defines a new media type to be used in conjuction with the current media type
when an RDAP extension needs to be described during HTTP content negotiation.

# RDAP-X: The RDAP With Extensions Media Type

The media type defined by this document is 'application/rdap-x+json'. This media
type has a parameter of "extensions" which is a whitespace-separated list of RDAP
extensions as defined in the IANA RDAP Extensions registry.

Here is an example:

    application/rdap-x+json;extensions="rdap_level_0 fred"
    
For readability, this document will refer to this media type, RDAP With Extensions,
as RDAP-X.

# Using The RDAP-X Media Type

[@!RFC7480] specifies the usage of 'application/json', 'application/rdap+json' or
both with HTTP Accept header. When using the media type defined by this document,
the 'application/rdap+json' media type MUST also be used in the Accept header.

An example:

    accept: application/rdap+json, application/rdap-x+json;extensions="rdap_level_0 fred"
    
When a server is programmed to understand the RDAP-X media type,
it SHOULD respond with this media type in the Content-Type header. By doing so,
clients will be able to detect if the server recognizes the media type. Otherwise,
the server will use the 'application/rdap+json' media type signalling to the client
that the RDAP-X media type is not recognized by the server.
This updates the usage of the Content-Type header with RDAP defined in RFC 7480,
but this usage is backward compatible.

When the RDAP-X media type is used in the Content-Type header, the
values in the media type's extension parameter SHOULD match the values in the `rdapConformance`
array in the return JSON. When there is a mismatch between extension parameters and
the `rdapConformance` array, clients SHOULD give preference to the `rdapConformance`
array.

# Usage in RDAP Links

[@!RFC9083, section 4.2] defines a link structure used in RDAP.

    {
      "value" : "https://example.com/context_uri",
      "rel" : "self",
      "href" : "https://example.com/target_uri",
      "hreflang" : [ "en", "ch" ],
      "title" : "title",
      "media" : "screen",
      "type" : "application/json"
    }

The type attribute signals to a client the expected media type of the resource
referenced in the href attribute, and some clients use this information to determine
if the URI in the href attribute should be dereferenced.

Servers MAY use the RDAP-X media type in the type attribute if a client
has negotiated content with the server using the RDAP-X media type, 
the resource referenced by the URI matches the RDAP-X media type, and
the resource referenced by the URI is served by a server compliant with this specification.
Otherwise, use of the `application/rdap+json` media type is RECOMMENDED when the URI
references RDAP resources. 

# IANA Considerations

Type name: application

Subtype name: rdapx+json

Required parameters: This media type has a parameter of "extensions" which is a whitespace-separated list of RDAP extensions as defined in the IANA RDAP Extensions registry.

Optional parameters:  N/A

Encoding considerations: See Section 3.1 of [@!RFC6839].

Security considerations: The media represented by this identifier does not have security considerations beyond that found in Section 12 of [@!RFC8259].

Interoperability considerations: There are no known interoperability problems regarding this media format.

Published specification: This document.

Applications that use this media type: Implementations of the Registration Data Access Protocol (RDAP) with Extensions.

Additional information: This media type is a product of the IETF REGEXT Working Group. The REGEXT charter, information on the REGEXT mailing list, and other documents produced by the REGEXT Working Group can be found at https://datatracker.ietf.org/wg/regext/.

Person & email address to contact for further information: IESG <iesg&ietf.org>

Intended usage: COMMON

Restrictions on usage: none

Author: Andy Newton

Change controller: IETF

Provisional Registration: No

{backmatter}

# Design Considerations

## Not Reusing the Existing Media Type

[@?RFC6838, section 4.3] strongly discourages the creation of new parameters on existing
media types to enable new features. As RDAP has always had extensions, it could be argued
that adding an "extensions" parameter to the existing `application/rdap+json` media type
is not adding a new feature to RDAP. However, the opposite could be argued that adding
the capability for clients to signal desired RDAP extensions is a new feature.

More practically, there is concern that adding a new parameter to the existing media
type would not be backward compatible with some server software. That is, servers
examining media types as exact string matches may incorrectly conclude that the existing
media type with an unknown, new parameter may not be the same as the existing media
type without parameters. A similar, though less likely, concern exists for clients.

As servers are required to handle multiple media types according to [@!RFC7480] and [@?RFC9110],
it therefore seems reasonable to conclude that defining a new media type for use with
the existing media type is best to preserve backward compatibility.

## Query Parameters Considered Harmful

Another design approach to communicating RDAP extensions from the client to the
server would be the use of URI query parameters:

```
https://rdap.example/domain/foo.example?extensions=fizzbuzz  
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

The RDAP ecosystem uses redirects in many situations. [@!RFC7480] discusses "aggregators", which
are RDAP servers used to help clients find authoritative RDAP servers using the RDAP bootstrap
registries. Redirects are also heavily used by the RIRs when IP addresses or autonomous
system numbers are transferred from one RIR to another.

Within HTTP, URI query parameters are not explicitly preserved during a redirect (probably
due to architecture considerations, see the section below). Specific to RDAP, [@!RFC7480]
instructs RDAP servers to ignore unknown query parameters and instructs clients not to
transform a URL of a redirect.

Therefore, query parameters denoting RDAP extensions will not survive redirects. This can
be readily observed in currently deployed RDAP servers:

```
curl -v https://rdap-bootstrap.arin.net/bootstrap/autnum/2830?extension=fizzbuzz    
```

To further demonstrate that query parameters do not survive redirects but that media types
do survive redirects, consider the code found [here](https://github.com/anewton1998/draft-regext-ext-json-media-type).
This code consists of a simple client and a simple server. The client sets both a new
media type and query parameters. The servers listens on two ports, redirecting the client
from a URL on the first port to a URL on the second port.

Here is the output of the client. It shows that the query parameters are not automatically
preserved but that the media type is automatically preserved.

```
2023-05-13T19:48:42.055902Z  INFO client: sending reqwest
2023-05-13T19:48:42.098605Z DEBUG reqwest::connect: starting new connection: http://127.0.0.1:3000/
2023-05-13T19:48:42.098696Z DEBUG hyper::client::connect::http: connecting to 127.0.0.1:3000
2023-05-13T19:48:42.098915Z DEBUG hyper::client::connect::http: connected to 127.0.0.1:3000
2023-05-13T19:48:42.099407Z DEBUG hyper::proto::h1::io: flushed 147 bytes
2023-05-13T19:48:42.100343Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-05-13T19:48:42.100392Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-05-13T19:48:42.100537Z DEBUG hyper::client::pool: pooling idle connection for ("http", 127.0.0.1:3000)
2023-05-13T19:48:42.100730Z DEBUG reqwest::async_impl::client: redirecting 'http://127.0.0.1:3000/ex1/domain/foo.example?foo=bar' to 'http://127.0.0.1:4000/ex2/domain/foo.example'
2023-05-13T19:48:42.100873Z DEBUG reqwest::connect: starting new connection: http://127.0.0.1:4000/
2023-05-13T19:48:42.100933Z DEBUG hyper::client::connect::http: connecting to 127.0.0.1:4000
2023-05-13T19:48:42.101157Z DEBUG hyper::client::connect::http: connected to 127.0.0.1:4000
2023-05-13T19:48:42.101667Z DEBUG hyper::proto::h1::io: flushed 202 bytes
2023-05-13T19:48:42.102419Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-05-13T19:48:42.102462Z DEBUG hyper::proto::h1::conn: incoming body is content-length (66 bytes)
2023-05-13T19:48:42.102533Z DEBUG hyper::proto::h1::conn: incoming body completed
2023-05-13T19:48:42.102634Z DEBUG hyper::client::pool: pooling idle connection for ("http", 127.0.0.1:4000)
2023-05-13T19:48:42.102642Z  INFO client: returned content type: "application/rdap-x;extensions=\"foo bar\""
2023-05-13T19:48:42.102664Z  INFO client: status code is 418 I'm a teapot
2023-05-13T19:48:42.102777Z  INFO client: response is {"errorCode":418,"title": "Your Beverage Choice is Not Available"}
```

Here is the output of the server. It show that the client, upon redirect, automatically sends the media type
but does not automatically preserve the query parameters.

```
2023-05-13T19:48:30.935786Z  INFO servers: starting server on port 3000
2023-05-13T19:48:30.935816Z  INFO servers: starting server on port 4000
2023-05-13T19:48:42.099642Z DEBUG hyper::proto::h1::io: parsed 2 headers
2023-05-13T19:48:42.099688Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-05-13T19:48:42.099847Z  INFO servers: Serving request from 127.0.0.1:45264
2023-05-13T19:48:42.099877Z  INFO servers: accept values: "application/rdap+json, application/rdap-x+json;extensions=\"foo bar\""
2023-05-13T19:48:42.099902Z  INFO servers: redirecting to server on port 4000
2023-05-13T19:48:42.100047Z DEBUG hyper::proto::h1::io: flushed 147 bytes
2023-05-13T19:48:42.101783Z DEBUG hyper::proto::h1::io: parsed 3 headers
2023-05-13T19:48:42.101811Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-05-13T19:48:42.101921Z  INFO servers: Serving request from 127.0.0.1:56798
2023-05-13T19:48:42.101940Z  INFO servers: accept values: "application/rdap+json, application/rdap-x+json;extensions=\"foo bar\""
2023-05-13T19:48:42.101953Z  INFO servers: responding with an unuseful error
2023-05-13T19:48:42.102096Z DEBUG hyper::proto::h1::io: flushed 207 bytes
2023-05-13T19:48:42.103705Z DEBUG hyper::proto::h1::conn: read eof
2023-05-13T19:48:42.103697Z DEBUG hyper::proto::h1::conn: read eof
```

Preservation of query parameters is not a common feature of HTTP client and server libraries,
whereas preservation of media types is common.

### Architectual Violations

As noted in [@?RFC3986], URI query parameters are meant to be part of the identity of the resource
being identified by a URI and pointed to by the location of a URL. RDAP extensions change
the portions of JSON returned by the server but are not intended to change the resource
being identified. That is, a domain registration is the same domain registration regardless
of whether the postal address in that domain registration is communicated via JCard or
a new RDAP extension for JSContact.

Changing how the content of a resource is conveyed is called content negotiation and
is discussed in detail in [@?RFC9110] using media types.

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

The usage with the `rdapx` media type would be:

```
application/rdapx+json;extensions="rdap_level_0 fizzbuzz__V_2_1"    
```

Readers should note that this scenario is provided to show design intent and is not
a full-fledged extension versioning design. Additionally, the new media type
defined in this document has utility with existing, opaquely versioned RDAP extensions
and does not depend on the definition of a new versioning scheme for RDAP extensions.
