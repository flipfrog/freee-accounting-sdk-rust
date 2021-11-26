# InvoiceIndexResponseInvoiceContents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 請求内容ID | 
**order** | Option<**i32**> | 順序 | 
**_type** | **String** | 行の種類 | 
**qty** | **f32** | 数量 | 
**unit** | Option<**String**> | 単位 | 
**unit_price** | **f32** | 単価 | 
**amount** | **i64** | 金額 | 
**vat** | **i32** | 消費税額 | 
**reduced_vat** | **bool** | 軽減税率税区分（true: 対象、false: 対象外） | 
**description** | Option<**String**> | 備考 | 
**account_item_id** | Option<**i32**> | 勘定科目ID | 
**account_item_name** | Option<**String**> | 勘定科目名 | 
**tax_code** | Option<**i32**> | 税区分コード | 
**item_id** | Option<**i32**> | 品目ID | 
**item_name** | Option<**String**> | 品目 | 
**section_id** | Option<**i32**> | 部門ID | 
**section_name** | Option<**String**> | 部門 | 
**tag_ids** | **Vec<i32>** |  | 
**tag_names** | **Vec<String>** |  | 
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_1_tag_name** | Option<**String**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_2_tag_name** | Option<**String**> | セグメント２ | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]
**segment_3_tag_name** | Option<**String**> | セグメント３ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


