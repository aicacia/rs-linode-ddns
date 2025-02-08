# GetLinodeConfigs200ResponseDataInnerHelpers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**devtmpfs_automount** | Option<**bool**> | Populates the `/dev` directory early during boot without `udev`.  Defaults to `false`. | [optional][default to false]
**distro** | Option<**bool**> | Helps maintain correct `inittab` or `upstart` console device. | [optional]
**modules_dep** | Option<**bool**> | Creates a modules dependency file for the kernel you run. | [optional]
**network** | Option<**bool**> | Set to `true` to automatically configure static networking. | [optional]
**updatedb_disabled** | Option<**bool**> | Set to `true` to disable the `updatedb` cron job to avoid disk thrashing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


