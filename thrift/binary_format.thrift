struct SpanContext {
  /** This field is unsigned but thrift does not support it */
  1: i64 trace_id

  /** This field is unsigned but thrift does not support it */
  2: i64 trace_id_high

  /** This field is unsigned but thrift does not support it */
  3: i64 span_id

  /** This field is unsigned but thrift does not support it */
  4: i64 parent_span_id

  5: bool sampled

  /** This field is unsigned but thrift does not support it */
  6: i64 flags

  7: map<string, string> baggage_items
}
