require 'date-utils'
mongoose = require 'mongoose'
Schema = mongoose.Schema

# Define the schema for the "snapshots" collection.
snapshotSchema = new Schema
  node_id:
    type: String
    required: true

  timestamp:
    type: Date
    required: true

  key:
    type: String
    index:
      unique: true

snapshotSchema.pre 'save', (next) ->
  @key = "#{@node_id}:#{@timestamp.toFormat('YYYY-MM-DD-HH24-MI-SS')}"
  next()

mongoose.model 'Snapshot', snapshotSchema

module.exports = mongoose.model 'Snapshot'