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

    application/rdapx+json extensions="rdap_level_0 fred"
    
# Using The RDAP With Extensions Media Type

RFC 7480 specifies tha usage of 'application/json', 'application/rdap+json' or
both withe HTTP Accept header. When using the media type defined by this document,
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