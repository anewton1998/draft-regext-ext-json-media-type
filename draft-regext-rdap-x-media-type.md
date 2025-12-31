%%%
Title = "The \"exts_list\" Parameter for the RDAP Media Type"
area = "Applications and Real-Time Area (ART)"
workgroup = "Registration Protocols Extensions (regext)"
abbrev = "rdap-x"
ipr= "trust200902"

[seriesInfo]
name = "Internet-Draft"
value = "draft-ietf-regext-rdap-x-media-type-05"
stream = "IETF"
status = "standard"
date = 2025-12-31T00:00:00Z

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

# The RDAP Media Type With "exts_list" Parameter {#parameter}

The RDAP media type, "application/rdap+json", may have an optional parameter named "exts_list". 
This parameter is a whitespace-separated list of RDAP
extension identifiers (as would be found in the "rdapConformance" array).

Here is an example:

    application/rdap+json;exts_list="rdap_level_0 exts fred"
    

# Using The "exts_list" Parameter {#using}

[@!RFC7480] specifies the usage of "application/json", "application/rdap+json" or
both with HTTP "accept" header. The "exts_list" parameter may only be used with
the "application/rdap+json" media type.

This is an example of the "accept" header using the RDAP media type with an "exts_list" parameter:

    accept: application/json;q=0.9, 
        application/rdap+json;exts_list="rdap_level_0 exts fred";q=1
    
If both a client and a server support the "exts_list" parameter, and the client requests
an extension that is unimplemented by the server, the server MUST respond with
only extensions included in the response by the server. This behavior is backwards-compatible as
RDAP clients should ignore unknown RDAP extensions as specified by [RFC9083].
Responding with an HTTP 406 Not Acceptable status code is NOT RECOMMENDED
because an RDAP client could interpret this status code to mean that the server does not
understand RDAP in its entirety.

The extensions requested by the client represent a hint for the server in determining the extensions to include the response.
If a server is required to use an extension in a response that was not requested by the client, the server MUST respond as if the client had requested the extension.
This behavior is backwards-compatible as RDAP clients should ignore unknown
extensions as specified by [@!RFC9083]. Responding with an HTTP 406 Not Acceptable status
code is NOT RECOMMENDED for the reason stated above.

When the "exts_list" parameter is used in the RDAP media type in the "content-type" header, the
values in the media type's "exts_list" parameter MUST match the values in the "rdapConformance"
array in the returned JSON. However, implementation experience has shown that some HTTP
server libraries do not support modification of the "content-type" header per query type.
Therefore, use of the "exts_list" parameter with the media type of the "content-type" header
is NOT REQUIRED. That is, when used in the "content-type" header, the values of the "exts_list"
parameter must match that of the "rdapConformance" array but server may opt to omit the
"exts_list" parameter from the media type in the "content-type" header.

The contents of the "exts_list" parameter mirrors the content of the
"rdapConformance" array in server responses. This includes the identifier "rdap_level_0", which is not
an extension identifier but an identifier for the base RDAP specifications. Servers MUST
follow the same rules for placing "rdap_level_0" in the content of the "exts_list"
parameter and the "rdapConformance" array. Clients MUST interpret an "exts_list"
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
does not require the usage of those extension identifiers in the "exts_list" parameter,
though clients SHOULD list the extension identifier in the "exts_list" parameter when using
other protocol elements of those extensions for better compatibility with servers
recognizing the "exts_list" parameter. Servers SHOULD NOT require the usage of extension
identifiers in the "exts_list" parameter when other extension protocol elements are used for
backwards-compatibility purposes.

## Extension Identifier

This document defines an RDAP extension using the identifier "exts".
This RDAP extension defines no additional RDAP queries or response structures.

The purpose of this RDAP extension is to allow servers to signal support for the "exts_list" parameter in
"rdapConformance" arrays of responses to "/help" (aka "service discovery").

## Examples

The following examples use the HTTP/1.1 message exchange syntax as seen in [@!RFC9110].

### Classic Negotiation

This example demonstrates the negotiation of the "application/rdap+json" media type
as defined in [@!RFC7480] using an RDAP "/help" query. This example also demonstrates
the negotiation in which a client does not support the "exts_list" parameter, but a server does support
the "exts_list" parameter.

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "exts" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }


### Negotiation of an RDAP Extension

In this example, both the client and server support the "exts_list" parameter and a fictional
extension of "foo".

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;exts_list="rdap_level_0 exts foo"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "exts", "foo" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }


### No Server Support for exts_list Parameter

In this example, only the client supports the "exts_list" parameter, along with a fictional
extension of "foo" by both.

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;exts_list="rdap_level_0 exts foo"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "foo" ],
      "notices" : [
        { "description" : [ "my content includes a trailing CRLF" ] } ] }

### Differing Extension Negotiation

In this example, both the client and server support the "exts_list" parameter. The client
supports the extensions "foo" and "bar" while the server only support "foo".

Client Request:

    GET /help HTTP/1.1
    accept: application/rdap+json;exts_list="rdap_level_0 exts foo bar"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "exts", "foo" ],
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
    accept: application/rdap+json;exts_list="rdap_level_0 exts versioning_0_2"

Server Response:

    HTTP/1.1 200 OK
    content-type: application/rdap+json

    { "rdapConformance" : [ "rdap_level_0", "exts", "versioning" ],
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

Usage of the "exts_list" parameter in the media type of the "type" attribute is allowed
but the "type" attribute as a whole is only a hint, as noted by [@!RFC8288]:

> The "type" attribute, when present, is a hint indicating what the
> media type of the result of dereferencing the link should be.  Note
> that this is only a hint; for example, it does not override the
> Content-Type header field of a HTTP response obtained by actually
> following the link.

Using the "exts_list" parameter in the media type of the "type" attribute in RDAP links
is NOT RECOMMENDED as it may describe client capabilities which the client may not
possess.

# Implementation Status

RFC EDITOR NOTE: Please remove this section and the reference to RFC 7942 prior to publication as an RFC.

This section records the status of known implementations of the protocol defined by this specification at the
time of posting of this Internet-Draft, and is based on a proposal described in RFC 7942 [@?RFC7942].
The description of implementations in this section is intended to assist the IETF in its decision processes in
progressing drafts to RFCs. Please note that the listing of any individual implementation here does not imply
endorsement by the IETF. Furthermore, no effort has been spent to verify the information presented here that
was supplied by IETF contributors. This is not intended as, and must not be construed to be, a catalog of
available implementations or their features. Readers are advised to note that other implementations may exist.

According to RFC 7942, "this will allow reviewers and working groups to assign due consideration to documents
that have the benefit of running code, which may serve as evidence of valuable experimentation and feedback
that have made the implemented protocols more mature. It is up to the individual working groups to use this information as they see fit".

## IIT-CNR/Registro.it RDAP Server

* Responsible Organization: Institute of Informatics and Telematics of National Research Council (IIT-CNR)/Registro.it

* Location: https://rdap.pubtest.nic.it/

* Description: This implementation includes support for RDAP queries using data from the public test environment of .it ccTLD.

* Level of Maturity: This is an "alpha" test implementation.

* Coverage: This implementation includes all the features described in this specification.

* Contact Information: Mario Loffredo, mario.loffredo@iit.cnr.it

## ICANN-RDAP Client

* Responsible Organization: Internet Corporation for Assigned Names and Numbers

* Location: https://github.com/icann/icann-rdap

* Description: This is a general purpose RDAP client, including client libraries, used directly as CLI and embedded into other software such as intrusion detection systems.

* Level of Maturity: This software is widely used, however the features of this specification are in a pre-release branch of the software.

* Coverage: This implementation includes all the features described in this specification.

* Contact Information: ICANN Global Support, globalsupport@icann.org

## ICANN-RDAP Server

* Responsible Organization: Internet Corporation for Assigned Names and Numbers

* Location: https://github.com/icann/icann-rdap

* Description: This is a general purpose RDAP server, including libraries, used for tested and embedded into production servers of some registry service providers.

* Level of Maturity: The features of this specification are in a pre-release branch of the software.

* Coverage: This implementation includes all the features described in this specification.

* Contact Information: ICANN Global Support, globalsupport@icann.org

# Security Considerations

As stated in (#using), this specification does not override the protocol elements of
RDAP security extensions, such as [@?RFC9560], nor does it override
the protocol elements of other security features of HTTP.

This specification does contrast with solutions using query parameters in that those
solutions require servers to blindly copy query parameters into redirect URLs in
situations where such copying could cause harm, such as copying an API key intended
for one server into the redirect URL of another server.

# IANA Considerations

## RDAP Extension Registry

The IETF requests the IANA to register the following extension in the RDAP Extensions Registry at [@RDAP-EXTENSIONS]:

    Extension identifier: exts

    Registry operator: ALL

    Published specification: [RFC Reference Once Published]

    Person & email address to contact for further information:
    The Internet Engineering Steering Group <iesg@ietf.org>

    Intended usage: COMMON

## Addition of Parameter to RDAP Media Type

This document defines the optional parameter "exts_list" for the media type "application/rdap+json"
as described in (#parameter).

The IETF requests the IANA to add this document as an additional reference to the IANA Media Type registry at [@MEDIA-TYPES] for
the media type "application/rdap+json".

# Acknowledgements

Pawel Kowalik provided extensive review of this document and conducted a study that forms the
basis of re-using the existing RDAP media type. James Mitchell has provided ideas and feedbacks that have contributed to
the content of this document based on his RDAP server implementation experience. Murray Kucherawy and Alexey Melnikov provided guidance on the use of media types and
media type parameters. Maarten Wullink and Jame Gould provided feedback that contributed to the content of this document. Mario Loffredo has provided valuable
feedback on the contents of this specification, including feedback from his implementation, that has helped to reduce interoperability
issues. 

{backmatter}

# Using the Vary Header {#vary_header}

Server implementers may want to consider using the "vary" header depending on the caching
behavior desired of shared caches (i.e. middleboxes, not client caches).

Consider the following scenario where user Bob and user Alice send queries to the same
RDAP server that is routed through a middlebox network element implementing a shared HTTP cache.

User Bob sends a query for the domain "example.com"
(http://regy.example/domain/example.com) without the "exts_list" parameter. The "accept" header sent
for Bob's query would be `accept: application/rdap+json` or `accept: application/json`.

User Alice later sends a query for the same domain, however her client uses the "exts_list" parameter. The "accept"
header sent for Alice's query might be `accept: application/rdap+json;exts_list="rdap_level_0 exts foo"`.

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

Therefore, query parameters denoting RDAP extensions should not survive redirects in RDAP, and in many real-world examples
they do not survie redirects. This can
be readily observed in currently deployed RDAP servers:

```
curl -v https://rdap-bootstrap.arin.net/bootstrap/autnum/2830?exts_list=fizzbuzz    
```

To further demonstrate that query parameters do not automatically survive redirects but that media types
do, consider the code found [here](https://github.com/anewton1998/draft-regext-ext-json-media-type).
This code consists of a simple client and a simple server. The client sets both a new
media type and query parameters. The servers listen on two ports, redirecting the client
from a URL on the first port to a URL on the second port.

Preservation of query parameters is not a guaranteed feature of HTTP client and server libraries,
whereas preservation of media types is much more likely to occur.

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

Usage of the "exts_list" media-type parameter does not require clients to conduct further processing of these
referrals, whereas a query parameter approach would require clients to process and de-conflict
any other query parameters if present.

Just as in the copy-and-paste scenario above, a referral with a query parameter representing the RDAP extensions understood
by the client (i.e., the client's capability to understand RDAP responses), may indicate to a server some capabilities the client
may not possess.

Consider a scenario in which a user has a client that does no support the JSContact extension (i.e., it only supports jCard) and encounters the
following referral using a query parameter:

    {
      "value" : "https://regy.example/domain/foo.example",
      "rel" : "related",
      "href" : "https://regr.example/domain/foo.example?exts_list=jscontact,
      "type" : "application/rdap+json"
    }

Now consider that the server at regr.example is in a transition phase between jCard and JSContact and serves only JSContact when it is told the client
understands JSContact. In this scenario, the user would not be capable of processing the contact information becuase their client
only understand jCard and the server has responded only with JSContact.

### Architectural Violations

[@?RFC3986, section 3.4] states the following:

> The query component contains non-hierarchical data that, along with
> data in the path component (Section 3.3), serves to identify a
> resource within the scope of the URI's scheme and naming authority
> (if any). 

Therefore, URI query parameters are meant to be part of the identity of the resource
being identified by a URI and pointed to by the location of a URL. RDAP extensions change
the portions of JSON returned by the server but are not intended to change the resource
being identified. That is, a domain registration is the same domain registration regardless
of whether the postal address in that domain registration is communicated via JCard or
a new RDAP extension for JSContact.

Changing how the content of a resource is conveyed is called content negotiation and
is discussed in detail in [@?RFC9110] using media types.

<reference anchor='RDAP-EXTENSIONS' target='https://www.iana.org/assignments/rdap-extensions/'>
    <front>
        <title>RDAP Extensions</title>
        <author>
            <organization>IANA</organization>
        </author>
    </front>
</reference>

<reference anchor='MEDIA-TYPES' target='https://www.iana.org/assignments/media-types/'>
    <front>
        <title>Media Types</title>
        <author>
            <organization>IANA</organization>
        </author>
    </front>
</reference>

