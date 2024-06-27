/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://streams.spec.whatwg.org/#readablestream

[Exposed=*]
interface _ReadableStream {
    [Throws]
    constructor(optional object underlyingSource, optional QueuingStrategy strategy = {});
};

enum ReadableStreamType { "bytes" };

typedef (ReadableStreamDefaultReader or ReadableStreamBYOBReader) ReadableStreamReader;

enum ReadableStreamReaderMode { "byob" };
