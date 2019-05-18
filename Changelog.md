# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2019-05-18
### Added
- Thrift over HTTP collector (`/api/v1/spans` format).

### Changed
- Kafka collector is a feature (enabled for backward compatibility).

### Deprecated
- Default features will no longer include `kafka_transport` from v0.2.0

## [0.1.2] - 2018-03-04
### Fixed
- Extract returns `Ok(None)` if the trace ID header is missing.

## 0.1.1 - 2018-03-03
### Fixed
- Fix readme example.
- Fix repo link.

## 0.1.0 - 2018-02-23
### Added
- Basic documentation.
- Encode logs as JSON objects.
- Implement HTTP headers injection/extraction.
- Implement thrift-based binary injection/extraction.
- Initial library structure.
- Kafka Collector implementation.
- TraceID implementation.
- Zipkin SpanContext implementation.


[Unreleased]: https://github.com/stefano-pogliani/opentracingrust-zipkin/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/stefano-pogliani/opentracingrust-zipkin/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/stefano-pogliani/opentracingrust-zipkin/compare/v0.1.1...v0.1.2
