%%%
Title = "Extensions Parameter for the RDAP Media Type"
area = "Applications and Real-Time Area (ART)"
workgroup = "Registration Protocols Extensions (regext)"
abbrev = "rdap-x"
updates = [7480]
ipr= "trust200902"

[seriesInfo]
name = "Internet-Draft"
value = "draft-ietf-regext-rdap-x-media-type-03"
stream = "IETF"
status = "standard"
date = 2024-04-03T00:00:00Z

[[author]]
initials="A."
surname="Newton"
fullname="Andy Newton"
organization="ICANN"
[author.address]
email = "andy@hxr.us"

[[author]]
initials="J."
surname="Singh"
fullname="Jasdip Singh"
organization="ARIN"
[author.address]
email = "jasdips@arin.net"

%%%

.# Abstract

This document defines a new parameter for the RDAP media type that can be used to describe RDAP content
with RDAP extensions. Additionally, this document describes the usage of this parameter
with RDAP for the purposes of signalling RDAP extensions during content
negotiation.

{mainmatter}

# Background

[@!RFC7480] defines the "application/rdap+json" media type to be used with RDAP. This
document defines a new parameter for this media type
when an RDAP extension needs to be described during HTTP content negotiation.

This parameter enables an RDAP
client to signal to an RDAP server the list of RDAP extensions supported by that client.
For example, an RDAP client that supports the "foo" extension may use this mechanism
as a signal to an RDAP server, thus allowing the server to respond with data using the "foo"
extension inside an RDAP response only when it can be assured the client can understand the extension.

By using this method, there is no need for every RDAP extension to define their own unique
signaling mechanism. Additionally, this method is designed to be backwards-compatible
with the deployed RDAP ecosystem (see (#design_considerations) for further information).

## Document Terms

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED",
"MAY", and "OPTIONAL" in this document are to be interpreted as
described in BCP 14 [@!RFC2119] [@!RFC8174] when, and only when, they
appear in all capitals, as shown here.

# The RDAP Media Type With Extensions Parameter

The RDAP media type, "application/rdap+json", may have an optional parameter named "extensions". 
This parameter is a whitespace-separated list of RDAP
extensions as defined in the IANA RDAP Extensions registry.

Here is an example:

    application/rdap+json;extensions="rdap_level_0 rdapExtensions1 fred"
    

# Using The Extensions Parameter {#using}

[@!RFC7480] specifies the usage of "application/json", "application/rdap+json" or
both with HTTP "accept" header. The "extensions" parameter may only be used with
the "application/rdap+json" media type.

This is an example of the "accept" header using the RDAP media type with an "extensions" parameter:

    accept: application/json;q=0.9, 
        application/rdap+json;extensions="rdap_level_0 rdapExtensions1 fred";q=1
    
When a server is programmed to understand the "extensions" parameter,
it MUST respond with the "extensions" parameter in media type in the "content-type" header. By doing so,
clients will be able to detect if the server recognizes the "extensions" parameter.
[@!RFC7480] requires that only the "application/rdap+json" media type be used in the
"content-type" header.

If both a client and a server support the "extensions" parameter, and the client requests
an extension that is unimplemented by the server, the server MUST respond with
only extensions implemented by the server. This behavior
is backwards-compatible as RDAP clients must ignore unknown extensions as specified by
[@!RFC9083]. Responding with an HTTP 406 Not Acceptable status code is NOT RECOMMENDED
because an RDAP client could interpret this status code to mean that the server does not
understand RDAP in its entirety.

Likewise, if a server is required to use an extension in a response that was not
requested by the client, the server MUST respond as if the client had requested
the extension. This behavior is backwards-compatible as RDAP clients must ignore unknown
extensions as specified by [@!RFC9083]. Responding with an HTTP 406 Not Acceptable status
code is NOT RECOMMENDED for the reason stated above.

When the "extensions" parameter is used in the RDAP media type in the "content-type" header, the
values in the media type's "extensions" parameter MUST match the values in the "rdapConformance"
array in the returned JSON.

Just as servers must not put extensions into the "rdapConformance" array for which
they do not support, servers MUST NOT list extensions in the "extensions" parameter for
which they do not support.

The contents of the "extensions" parameter mirrors the content of the
"rdapConformance" array in server responses. This includes the identifier "rdap_level_0", which is not
an extension identifier but an identifier for the base RDAP specifications. Servers MUST
follow the same rules for placing "rdap_level_0" in the content of the "extensions"
parameter and the "rdapConformance" array. Clients MUST interpret an "extensions"
parameter without "rdap_level_0" or one of its successor identifiers (e.g. "rdap_level_1")
in the same manner as the interpretation of the "rdapConformance" array without
"rdap_level_0" or one of its successors.

Nothing in this specification sidesteps or obviates the HTTP content negotiation defined
in [@!RFC9110] for RDAP.

Likewise, nothing in this specification sidesteps or obviates the HTTP caching mechanisms
defined in [@!RFC9110]. Further advice on the "vary" header can be found in (#vary_header).

Some RDAP extensions, such as [@?RFC9560], have other protocol elements (e.g. extension-specific query parameters)
passed from the client to the server, and the presence of these protocol elements may be
used by servers to determine a client's capability to handle the related RDAP extension(s). This specification
does not require the usage of those extension identifiers in the "extensions" parameter,
though clients SHOULD list the extension identifier in the "extensions" parameter when using
other protocol elements of those extensions for better compatibility with servers
recognizing the "extensions" parameter. Servers SHOULD NOT require the usage of extension
identifiers in the "extensions" parameter when other extension protocol elements are used for
backwards-compatibility purposes.

## Extension Identifier

This document defines an RDAP "profile" extension using the identifier "rdapExtensions1".
This RDAP extension defines no additional RDAP queries or response structures.

The purpose of this RDAP extension is to allow servers to signal support for the "extensions" parameter in
"rdapConformance" arrays of responses to "/help" (aka "service discovery").

## Examples

The following examples use the HTTP/1.1 message exchange syntax as seen in [@!RFC9110].

### Classic Negotiation

This example demonstrates the negotiation of the "application/rdap+json" media type
as defined in [@!RFC7480] using an RDAP "/help" query. This example also demonstrates
the negotiation in which a client does not support the "extensions" parameter but a server does support
the "extensions" parameter.

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json;extensions="rdap_level_0 rdapExtensions1"

    { "rdapConformance" : [ "rdap_level_0", "rdapExtensions1" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }


### Negotiation of an RDAP Extension

In this example, both the client and server support the "extensions" parameter and a fictional
extension of "foo".

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo"

    { "rdapConformance" : [ "rdap_level_0", "rdapExtensions1", "foo" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }

### No Server Support for Extensions Parameter

In this example, only the client supports the "extensions" parameter, along with a fictional
extension of "foo" by both.

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "foo" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }

### Differing Extension Negotiation

In this example, both the client and server support the "extensions" parameter. The client
supports the extensions "foo" and "bar" while the server only support "foo".

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo bar"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo"

    { "rdapConformance" : [ "rdap_level_0", "rdapExtensions1", "foo" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }

### Extension Versioning and Meta-data {#versioning}

For scenarios where the "versioning" extension, as defined by [@?I-D.ietf-regext-rdap-versioning],
is used, the extension identifiers in the client request may not be exact or case-insensitive matches for the
extension identifiers in the server response (unlike scenarios where the "versioning" extension is not used).
That is, the extension identifiers used by the client have appended versioning information, but the
extension identifiers returned by the server do not have appended versioning information (such information
is in the "versioning" JSON).

Client Request:

    GET /domain/example.com HTTP/1.1
    accept: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 versioning_0_2"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 versioning"

    { "rdapConformance" : [ "rdap_level_0", "rdapExtensions1", "versioning" ],
      "objectClassName": "domain",
      "ldhName": "example.com",
      "versioning": [ {
        "extension": "versioning",
        "type": "semantic",
        "version": "versioning_0_2" } ]
    }

Servers might also use the "versioning" extension to describe meta-data about
supported extensions even if the servers do not explicitly support extension versioning.

# Usage in RDAP Links {#links}

[@!RFC9083, section 4.2] defines a link structure used in RDAP.

    {
      "value": "https://example.com/context_uri",
      "rel": "self",
      "href": "https://example.com/target_uri",
      "hreflang": [ "en", "ch" ],
      "title": "title",
      "media": "screen",
      "type": "application/json"
    }

The type attribute signals to a client the expected media type of the resource
referenced in the href attribute, and some clients use this information to determine
if the URI in the href attribute should be de-referenced.

Usage of the "extensions" parameter in the media type of the "type" attribute is NOT RECOMMENDED
as clients are under obligation to use the "extensions" parameter as described in (#using). That is,
clients will populate the contents of the "extensions" parameter according to (#using) regardless of
its usage in the link.


# Security Considerations

As stated in (#using), this specification does not override the protocol elements of
RDAP security extensions, such as [@?RFC9560], nor does it override
the protocol elements of other security features of HTTP.

This specification does contrast with solutions using query parameters in that those
solutions require servers to blindly copy query parameters into redirect URLs in
situations where such copying could cause harm, such as copying an API key intended
for one server into the redirect URL of another server.

# IANA Considerations

The IETF requests the IANA to register the following extension in the RDAP Extensions Registry at [@RDAP-EXTENSIONS]:

    Extension identifier: rdapExtensions1

    Registry operator: ALL

    Published specification: [RFC Reference Once Published]

    Person & email address to contact for further information:
    The Internet Engineering Steering Group <iesg@ietf.org>

    Intended usage: COMMON

# Acknowledgements

Pawel Kowalik provided extensive review of this document and conducted a study that forms the
basis of re-using the existing RDAP media type. Mario Loffredo and James Mitchell have provided ideas and feedbacks that have contributed to
the content of this document. Murray Kucherawy and Alexey Melnikov provided guidance on the use of media types and
media type parameters.

{backmatter}

# Using the Vary Header {#vary_header}

Server implementers may want to consider using the "vary" header depending on the caching
behavior desired of shared caches (i.e. middleboxes, not client caches).

Consider the following scenario where user Bob and user Alice send queries to the same
RDAP server that is routed through a middlebox network element implementing a shared HTTP cache.

User Bob sends a query for the domain "example.com"
(http://regy.example/domain/example.com) without the "extensions" parameter. The "accept" header sent
for Bob's query would be `accept: application/rdap+json` or `accept: application/json`.

User Alice later sends a query for the same domain, however her client uses the "extensions" parameter. The "accept"
header sent for Alice's query might be `accept: application/rdap+json;extensions="rdap_level_0 rdapExtensions1 foo"`.

If no "vary" header is set in the response for these queries, the shared cache will compare only
the URL of the query when processing cache items and therefore user Bob and user Alice would receive
the same answer. In other words, since both queried "http://regy.example/domain/example.com" the shared
cache would return the answer of the first query to the second query and all other subsequent queries
until the item expired out of the cache.

If server implementers do not desire this behavior and would signal that caches consider each query
separately, servers should also return a "vary: accept" header to inform the cache that the "accept"
header should also be considered when processing cache items. Server implementers should also
consult [@!RFC9110] regarding caching and other uses of the "vary" header.

# Design Considerations {#design_considerations}

## Reusing the Existing Media Type

Earliest versions of this document specified a new media type because the authors believed
the addition of new parameter on the existing RDAP media type may be backwards-incompatible
with many RDAP servers. However, a study conducted by Pawel Kowalik concluded that 99.65%
of RDAP servers are compatible with a new parameter on the existing RDAP media type.

Additionally, [@?RFC2045] requires that the server ignore unknown parameters.

## Inappropriate Use of Query Parameters

Another design approach to communicating RDAP extensions from the client to the
server would be the use of URI query parameters:

```
https://rdap.example/domain/foo.example?extensions=fizzbuzz  
```

However, there are a few problems with using query parameters for this scenario.
Some of these problems are specific to RDAP and are also documented in
[@?I-D.ietf-regext-rdap-extensions]. The following sections also describe the
problems.

### Copy and Paste

Consider two RDAP users, Alice and Bob. Alice has an RDAP client that supports
the extension "fizzbuzz", and Bob has an RDAP client that does not support this
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
transform the URL of a redirect.

Therefore, query parameters denoting RDAP extensions should not survive redirects in RDAP. This can
be readily observed in currently deployed RDAP servers:

```
curl -v https://rdap-bootstrap.arin.net/bootstrap/autnum/2830?extension=fizzbuzz    
```

To further demonstrate that query parameters do not automatically survive redirects but that media types
do, consider the code found [here](https://github.com/anewton1998/draft-regext-ext-json-media-type).
This code consists of a simple client and a simple server. The client sets both a new
media type and query parameters. The servers listen on two ports, redirecting the client
from a URL on the first port to a URL on the second port.

Here is the output of the client. It shows that the query parameters are not automatically
preserved but that the media type is automatically preserved.

```
2024-01-05T11:15:34.380989Z  INFO client: sending reqwest to http://127.0.0.1:3000/ex1/domain/foo.example?foo&bar
2024-01-05T11:15:34.431386Z  INFO client: returned content type: "application/rdap+json;extensions=\"foo bar\""
2024-01-05T11:15:34.431413Z  INFO client: status code is 418 I'm a teapot
2024-01-05T11:15:34.431476Z  INFO client: response is {"errorCode":418,"title": "Your Beverage Choice is Not Available"}
```

Here is the output of the server. It shows that the client, upon redirect, automatically sends the media type
but does not automatically preserve the query parameters.

```
2024-01-05T11:15:31.071936Z  INFO servers: starting server on port 3000
2024-01-05T11:15:31.071961Z  INFO servers: starting server on port 4000
2024-01-05T11:15:34.429595Z  INFO servers: [redirecting server] Serving request from 127.0.0.1:60260
2024-01-05T11:15:34.429648Z  INFO servers: [redirecting server] received query parameters: 'bar', 'foo'
2024-01-05T11:15:34.429682Z  INFO servers: [redirecting server] client signaled RDAP extension 'foo'
2024-01-05T11:15:34.429693Z  INFO servers: [redirecting server] client signaled RDAP extension 'bar'
2024-01-05T11:15:34.429704Z  INFO servers: [redirecting server] redirecting to http://127.0.0.1:4000/ex2/domain/foo.example
2024-01-05T11:15:34.430940Z  INFO servers: [authoritative server] Serving request from 127.0.0.1:40840
2024-01-05T11:15:34.430967Z  INFO servers: [authoritative server] received query parameters:
2024-01-05T11:15:34.430983Z  INFO servers: [authoritative server] client signaled RDAP extension 'foo'
2024-01-05T11:15:34.430989Z  INFO servers: [authoritative server] client signaled RDAP extension 'bar'
2024-01-05T11:15:34.430995Z  INFO servers: [authoritative server] responding with an unuseful error
```

Preservation of query parameters is not a guaranteed feature of HTTP client and server libraries,
whereas preservation of media types is much more likely.

### Referral Compatibility

It is common in the RDAP ecosystem to link from one RDAP resource to another. These are typically
conveyed in the link structure defined in [@?RFC9083, section 4.2] and use the "application/rdap+json"
media type. One common usage is to link to a domain registration in a domain registrar from
a domain registration in a domain registry.

    {
      "value" : "https://regy.example/domain/foo.example",
      "rel" : "related",
      "href" : "https://regr.example/domain/foo.example",
      "type" : "application/rdap+json"
    }

Usage of the "extensions" parameter does not require clients to conduct further processing of these
referrals, whereas a query parameter approach would require clients to process and de-conflict
any other query parameters if present.

### Architectural Violations

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
two designs, the design that does not violate architecture should be preferred when all
other considerations are equal.

<reference anchor='RDAP-EXTENSIONS' target='https://www.iana.org/assignments/rdap-extensions/'>
    <front>
        <title>RDAP Extensions</title>
        <author>
            <organization>IANA</organization>
        </author>
    </front>
</reference>

