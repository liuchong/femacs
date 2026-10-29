# CLN-20261027-72 · TLS共享声明归位

P-401恢复TLS时，进程头与库实现必须使用同一不透明指针、阶段和错误类型。
原先这些名称只在禁用分支的实现文件中；现在25项定义从 `src/gnutls.rs`
归入实际共享头，调用方通过头引用，不保留第二套同名类型。没有删除源码文件，
没有删除缓存或声称释放磁盘空间；这是一批源码声明归位，不是过时功能退役。

原始Git基线 `1af56bb8d778ee1588d5d43c9270ae2e6d43892d`。逐项原定义片段SHA及四个原文件的
完整SHA/字节/Git blob见[身份清单](../manifests/2026-10-27-72-tls-declaration-consolidation.json)，
清单SHA256 `ae099d5d54aafc72f543ba52a286d43b7a26c3208eba5b7b8aa21e67d8efed4c`。当前TLS正文/配置/依赖接线仍属未交付工作，不能把下表
迁移完成或静态检查当作TLS编译、接口布局、GC、连接或兼容验收通过。

| 原名称 | 共享定义落点 | 原定义片段SHA256 |
| --- | --- | --- |
| `gnutls_session_int` | `src/include/gnutls-types.rs` | `a31016a5caafe299ea1a27664cc1b2a381090616367739d9fa06f991ddf84fea` |
| `gnutls_session_t` | `src/include/gnutls-types.rs` | `e2cc06a8bdb24b8269b2562fa4b5ff8c9ce0bc10cb84c9c909b4f8eea1b36e37` |
| `gnutls_x509_crt_int` | `src/include/gnutls-types.rs` | `4125ffe317957504d11409ee023ae4b219b4f7d3160336b691ad601fbe47e467` |
| `gnutls_x509_crt_t` | `src/include/gnutls-types.rs` | `fd99f95a741939540c12c5fd9aa9bd8043c134829f6416eccdc168ce6e80f5a1` |
| `gnutls_certificate_credentials_st` | `src/include/gnutls-types.rs` | `ba3429de3aef0a673a860a7132a220360a2bc1971e83b7139c3d1decea94de30` |
| `gnutls_certificate_credentials_t` | `src/include/gnutls-types.rs` | `d452a48f0307cd137f841e88343fef7e2b6afca37bc7b36ebb22734cd5ffe548` |
| `gnutls_certificate_client_credentials` | `src/include/gnutls-types.rs` | `460b65ad9f0ab15741b17f9f7e2e1cf359eac8d514c7bb8da0e1e8f8c806c044` |
| `gnutls_anon_client_credentials_st` | `src/include/gnutls-types.rs` | `86c61fe91850951dc6b21090f568dbb9033c652c25c9236b1a56e4ce4cc44e89` |
| `gnutls_anon_client_credentials_t` | `src/include/gnutls-types.rs` | `d2cf8bb64591344793177fd8fa5de5d500f8f33cc42d6742b5149df366f65bc8` |
| `gnutls_initstage_t` | `src/include/gnutls-types.rs` | `328ae21e6bb2782443cad7186cb3c9f06bef54b5d84a526d6cf06d08d5f6715f` |
| `GNUTLS_STAGE_EMPTY` | `src/include/gnutls-types.rs` | `e7c56294fbc373ebef6192a80b8f81bc6c9253d5e6709cc262de207094ebd474` |
| `GNUTLS_STAGE_CRED_ALLOC` | `src/include/gnutls-types.rs` | `c1a954bd3e046efebcb5ad261e8bc88f22aca62f80b774828f22a5871a396a06` |
| `GNUTLS_STAGE_FILES` | `src/include/gnutls-types.rs` | `8a4c71c53f5c5c371d2496b02f7dd7b0f5f5b5c8a1a635b0a590886760727926` |
| `GNUTLS_STAGE_CALLBACKS` | `src/include/gnutls-types.rs` | `247770989d63af98b37aeeda4da84c744d33dc7f23d3fb813cc777ac3bd34fdf` |
| `GNUTLS_STAGE_INIT` | `src/include/gnutls-types.rs` | `f16dba6df33172a49e0b87651b49609245bfd0f13587df2e4cd2afb80b1b15a6` |
| `GNUTLS_STAGE_PRIORITY` | `src/include/gnutls-types.rs` | `c9085b835ef3ca33b0f8f76a58b3c96dab64872bba04d7b85905e26668c6f165` |
| `GNUTLS_STAGE_CRED_SET` | `src/include/gnutls-types.rs` | `5ed0018ad6df7e5d860ade45edff8a694242de99d2f265dae1eca05064b88093` |
| `GNUTLS_STAGE_HANDSHAKE_CANDO` | `src/include/gnutls-types.rs` | `a146c64ead93664f5095137a7e03d06e32b552d6ede80399fe20f59abd93123a` |
| `GNUTLS_STAGE_TRANSPORT_POINTERS_SET` | `src/include/gnutls-types.rs` | `2dfef1a6b00618780ba523edaf82c874ef47ef9b89618e2973e0cc4645bd4a31` |
| `GNUTLS_STAGE_HANDSHAKE_TRIED` | `src/include/gnutls-types.rs` | `02f4733cb0fba94f535b836b1dad911855d6471b762f50d17fa25a8071fb4150` |
| `GNUTLS_STAGE_READY` | `src/include/gnutls-types.rs` | `75cc71e3292280be803cb4d9390126681097f455c4e58d45a142e7e26c2db2aa` |
| `GNUTLS_EMACS_HANDSHAKES_LIMIT` | `src/include/gnutls-types.rs` | `1102c2bd0013891cf03fa04f2464b66771e96d079568ee2baa6f3d267a00e706` |
| `GNUTLS_EMACS_ERROR_NOT_LOADED` | `src/include/gnutls-types.rs` | `ddc22ea1778cd8492a185a40c9108d2eb314bfc179b0fcd4ec7774b27dfba15f` |
| `GNUTLS_EMACS_ERROR_INVALID_TYPE` | `src/include/gnutls-types.rs` | `6c8556f4353d14a51dda22ce26595c9208e0483decc98497224f3c3c59f4fff2` |
| `GNUTLS_INITSTAGE` | `src/include/gnutls.rs` | `031183d9143d93c3cb461f9de268c6d830e5b9a54cb28dbe03db03c663d90719` |

首轮真实编译暴露两个进程本地读写原型错误返回 `ssize_t`，遮蔽共享头的
实际 `ptrdiff_t` 原型；另有重复 `Fgnutls_boot` 声明与现有subr声明冲突。
本批撤回这3份重复原型，唯一声明分别归共享TLS头和既有subr声明。
清单另附原文件和删除片段身份；函数实现与行为没有因此删除。
实际接口诊断已通过99项C/Rust布局、位域、枚举观测一致性和103项外部
函数原型编译断言。此前失败完整保留；这仍不是整体TLS功能验收。

## 恢复与边界

原片段在本机P401施工前快照及Git原始blob中保留，清单提供逐项身份。
按名称/原片段哈希定位最小恢复；不能直接覆盖整个实现或进程文件，那里还有
其他未提交工作。若撤回整个P401，应一并协调共享类型、进程布局/位域、实际
消费者、目标配置、构建/打包和资源身份，不能只恢复一份头产生不一致状态。
不用全量复制仓库或修改只读参照来恢复。正式交付时随完整TLS语义单元签名提交，
功能/接口及保护性验证结果由同一阶段实际证据补充；当前尚未完成该验收。

[清理台账](../LOG.md)


2026-10-29 source-delivery update: the historical construction readings above
remain historical. The declaration changes are included in the complete
experimental runtime foundation group, whose exact proposed tree built four
macOS baseline products and passed the limited integration and protection
checks in [the foundation report](../../releases/2026-10-29-runtime-foundation.md).
This records source consolidation; it is not physical deletion, full compatibility
or native/generational-GC performance acceptance.
