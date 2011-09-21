mongoose = require 'mongoose'
Schema = mongoose.Schema

# Define the schema for the "processes" collection.
Process = new Schema
  snapshot:
    type: Schema.ObjectId
    ref: 'Snapshot'
    required: true

  user: String
  pid: Number
  cpu_usage: Number
  memory_usage: Number
  virtual_memory_size: Number
  residental_set_size: Number
  tty: String
  state: String
  started_at: Date
  cpu_time: String
  command: String

mongoose.model 'Process', Process
module.exports = mongoose.model 'Process'