## 元の特徴量

| 英名                          | 日本語名        | 詳細説明                                                                                              |
|-----------------------------|-------------|---------------------------------------------------------------------------------------------------|
| age_approx                  | おおよその年齢     | 画像撮影時の患者のおおよその年齢                                                                                  |
| clin_size_long_diam_mm      | 最大径 (mm)    | 病変の最大径 (mm)                                                                                       |
| tbp_lv_A                    | A 内部病変      | 病変内のAレベル                                                                                          |
| tbp_lv_Aext                 | A 外部病変      | 病変外のAレベル                                                                                          |
| tbp_lv_B                    | B 内部病変      | 病変内のBレベル                                                                                          |
| tbp_lv_Bext                 | B 外部病変      | 病変外のBレベル                                                                                          |
| tbp_lv_C                    | C 内部病変      | 病変内のCレベル                                                                                          |
| tbp_lv_Cext                 | C 外部病変      | 病変外のCレベル                                                                                          |
| tbp_lv_H                    | 色相 (内部)     | 病変内の色相                                                                                            |
| tbp_lv_Hext                 | 色相 (外部)     | 病変外の色相                                                                                            |
| tbp_lv_L                    | 明度 (内部)     | 病変内の明度                                                                                            |
| tbp_lv_Lext                 | 明度 (外部)     | 病変外の明度                                                                                            |
| tbp_lv_areaMM2              | 面積 (mm^2)   | 病変の面積 (mm^2)                                                                                      |
| tbp_lv_area_perim_ratio     | 周囲長対面積比     | 周囲長と面積の比率                                                                                         |
| tbp_lv_color_std_mean       | 色不規則性       | 病変内の色のばらつき                                                                                        |
| tbp_lv_deltaA               | A コントラスト    | 病変内外のAコントラスト                                                                                      |
| tbp_lv_deltaB               | B コントラスト    | 病変内外のBコントラスト                                                                                      |
| tbp_lv_deltaL               | 明度コントラスト    | 病変内外の明度コントラスト                                                                                     |
| tbp_lv_deltaLBnorm          | コントラスト比     | 病変と周辺皮膚のコントラスト比                                                                                   |
| tbp_lv_eccentricity         | 偏心率         | 病変の偏心率                                                                                            |
| tbp_lv_minorAxisMM          | 最小径 (mm)    | 病変の最小径 (mm)                                                                                       |
| tbp_lv_nevi_confidence      | ネビス信頼度      | ネビスの信頼度スコア. ネビス信頼度スコア（0-100）は、約57,000の皮膚科医がラベル付けしたデータを基に訓練されたニューラルネットワークが、病変が良性のほくろである確率を推定したものです |
| tbp_lv_norm_border          | 境界不規則性      | 境界の不規則性スコア                                                                                        |
| tbp_lv_norm_color           | 色のばらつき      | 色のばらつきスコア                                                                                         |
| tbp_lv_perimeterMM          | 周囲長 (mm)    | 病変の周囲長 (mm)                                                                                       |
| tbp_lv_radial_color_std_max | 色の非対称性      | 病変内の色の非対称性                                                                                        |
| tbp_lv_stdL                 | 明度標準偏差      | 病変内の明度の標準偏差                                                                                       |
| tbp_lv_stdLExt              | 明度標準偏差 (外部) | 病変外の明度の標準偏差                                                                                       |
| tbp_lv_symm_2axis           | 対称性         | 病変の対称性スコア                                                                                         |
| tbp_lv_symm_2axis_angle     | 対称軸角度       | 対称軸の角度                                                                                            |
| tbp_lv_x                    | X座標         | 病変のX座標                                                                                            |
| tbp_lv_y                    | Y座標         | 病変のY座標                                                                                            |
| tbp_lv_z                    | Z座標         | 病変のZ座標                                                                                            |

## 新しく作成した特徴量

| 英名                             | 日本語名           | 詳細説明                     | 計算方法                                                                                                                    |
|--------------------------------|----------------|--------------------------|-------------------------------------------------------------------------------------------------------------------------|
| lesion_size_ratio              | 病変サイズ比         | 病変の最小径と最大径の比率            | tbp_lv_minorAxisMM（最小径）をclin_size_long_diam_mm（最大径）で割る                                                                  |
| lesion_shape_index             | 病変形状指数         | 病変の面積と周囲長の平方の比率          | tbp_lv_areaMM2（面積）を(tbp_lv_perimeterMM（周囲長）の平方)で割る                                                                      |
| hue_contrast                   | 色相コントラスト       | 病変内外の色相の差の絶対値            | tbp_lv_H（色相）からtbp_lv_Hext（外部色相）を引いた絶対値                                                                                  |
| luminance_contrast             | 明度コントラスト       | 病変内外の明度の差の絶対値            | tbp_lv_L（明度）からtbp_lv_Lext（外部明度）を引いた絶対値                                                                                  |
| lesion_color_difference        | 色差             | 病変内外の色差の総和               | (tbp_lv_deltaA（Aコントラスト）の平方 + tbp_lv_deltaB（Bコントラスト）の平方 + tbp_lv_deltaL（明度コントラスト）の平方)の平方根                                |
| border_complexity              | 境界複雑度          | 境界不規則性と対称性の総和            | tbp_lv_norm_border（境界不規則性）とtbp_lv_symm_2axis（対称性）の和                                                                     |
| color_uniformity               | 色の均一性          | 色の標準偏差と放射状色の標準偏差の比率      | tbp_lv_color_std_mean（色不規則性）をtbp_lv_radial_color_std_max（放射状色の標準偏差）で割る                                                  |
| 3d_position_distance           | 3D位置距離         | 病変の3D位置座標の距離             | (tbp_lv_x（X座標）の平方 + tbp_lv_y（Y座標）の平方 + tbp_lv_z（Z座標）の平方)の平方根                                                            |
| perimeter_to_area_ratio        | 周囲長対面積比        | 周囲長と面積の比率                | tbp_lv_perimeterMM（周囲長）をtbp_lv_areaMM2（面積）で割る                                                                           |
| lesion_visibility_score        | 病変可視性スコア       | コントラスト比と色のばらつきの総和        | tbp_lv_deltaLBnorm（コントラスト比）とtbp_lv_norm_color（色のばらつき）の和                                                                 |
| combined_anatomical_site       | 結合解剖学的部位       | 一般的な解剖部位と位置の組み合わせ        | anatom_site_general（解剖部位）とtbp_lv_location（位置）を"_"で結合                                                                    |
| symmetry_border_consistency    | 対称性境界の一貫性      | 対称性スコアと境界不規則性の積          | tbp_lv_symm_2axis（対称性）とtbp_lv_norm_border（境界不規則性）の積                                                                     |
| color_consistency              | 色の一貫性          | 明度の標準偏差と外部明度の比率          | tbp_lv_stdL（明度標準偏差）をtbp_lv_Lext（外部明度）で割る                                                                                |
| size_age_interaction           | サイズと年齢の相互作用    | 病変の最大径と年齢の積              | clin_size_long_diam_mm（最大径）とage_approx（おおよその年齢）の積                                                                       |
| hue_color_std_interaction      | 色相と色標準偏差の相互作用  | 色相と色の標準偏差の積              | tbp_lv_H（色相）とtbp_lv_color_std_mean（色不規則性）の積                                                                             |
| lesion_severity_index          | 病変重症度指数        | 境界不規則性、色のばらつき、偏心率の平均     | (tbp_lv_norm_border（境界不規則性）+ tbp_lv_norm_color（色のばらつき）+ tbp_lv_eccentricity（偏心率）)を3で割る                                  |
| shape_complexity_index         | 形状複雑度指数        | 境界複雑度と病変形状指数の総和          | border_complexity（境界複雑度）とlesion_shape_index（病変形状指数）の和                                                                   |
| color_contrast_index           | 色コントラスト指数      | 色差とコントラスト比の総和            | tbp_lv_deltaA（Aコントラスト）+ tbp_lv_deltaB（Bコントラスト）+ tbp_lv_deltaL（明度コントラスト）+ tbp_lv_deltaLBnorm（コントラスト比）の和                  |
| log_lesion_area                | 病変面積の対数        | 病変面積の対数値                 | tbp_lv_areaMM2（面積）に1を足してからその対数を取る                                                                                       |
| normalized_lesion_size         | 正規化病変サイズ       | 最大径と年齢の比率                | clin_size_long_diam_mm（最大径）をage_approx（おおよその年齢）で割る                                                                      |
| mean_hue_difference            | 平均色相差          | 色相の平均差                   | (tbp_lv_H（色相）とtbp_lv_Hext（外部色相）の和)を2で割る                                                                                 |
| std_dev_contrast               | 標準偏差コントラスト     | 色差の標準偏差                  | (tbp_lv_deltaA（Aコントラスト）の平方 + tbp_lv_deltaB（Bコントラスト）の平方 + tbp_lv_deltaL（明度コントラスト）の平方)を3で割ってからその平方根を取る                    |
| color_shape_composite_index    | 色と形状の総合指数      | 色のばらつき、周囲長対面積比、対称性の平均    | (tbp_lv_color_std_mean（色不規則性）+ tbp_lv_area_perim_ratio（周囲長対面積比）+ tbp_lv_symm_2axis（対称性）)を3で割る                           |
| 3d_lesion_orientation          | 3D病変の向き        | 3D座標の角度                  | tbp_lv_y（Y座標）をtbp_lv_x（X座標）で割ってその逆正接関数を取る                                                                               |
| overall_color_difference       | 総色差            | 色差の平均                    | (tbp_lv_deltaA（Aコントラスト）+ tbp_lv_deltaB（Bコントラスト）+ tbp_lv_deltaL（明度コントラスト）)を3で割る                                          |
| symmetry_perimeter_interaction | 対称性と周囲長の相互作用   | 対称性スコアと周囲長の積             | tbp_lv_symm_2axis（対称性）とtbp_lv_perimeterMM（周囲長）の積                                                                        |
| comprehensive_lesion_index     | 総合病変指数         | 境界不規則性、偏心率、色のばらつき、対称性の平均 | (tbp_lv_area_perim_ratio（周囲長対面積比）+ tbp_lv_eccentricity（偏心率）+ tbp_lv_norm_color（色のばらつき）+ tbp_lv_symm_2axis（対称性）)を4で割る    |
| **color_age_interaction**      | 色と年齢の相互作用      | 色不規則性と年齢の積               | tbp_lv_color_std_mean（色不規則性）とage_approx（おおよその年齢）の積                                                                      |
| **area_age_interaction**       | 面積と年齢の相互作用     | 病変の面積と年齢の積               | tbp_lv_areaMM2（面積）とage_approx（おおよその年齢）の積                                                                                |
| color_variance_ratio           | 色の分散比率         | 色不規則性と外部明度の比率            | tbp_lv_color_std_mean（色不規則性）をtbp_lv_stdLExt（外部明度標準偏差）で割る                                                                |
| border_color_interaction       | 境界色相互作用        | 境界不規則性と色のばらつきの積          | tbp_lv_norm_border（境界不規則性）とtbp_lv_norm_color（色のばらつき）の積                                                                  |
| size_color_contrast_ratio      | サイズと色コントラスト比   | 最大径とコントラスト比の比率           | clin_size_long_diam_mm（最大径）をtbp_lv_deltaLBnorm（コントラスト比）で割る                                                              |
| age_normalized_nevi_confidence | 年齢正規化ネビス信頼度    | ネビス信頼度と年齢の比率             | tbp_lv_nevi_confidence（ネビス信頼度）をage_approx（おおよその年齢）で割る                                                                   |
| color_asymmetry_index          | 色の非対称性指数       | 色の非対称性と対称性の積             | tbp_lv_radial_color_std_max（色の非対称性）とtbp_lv_symm_2axis（対称性）の積                                                            |
| 3d_volume_approximation        | 3Dボリューム近似      | 面積と3D位置座標の距離の積           | tbp_lv_areaMM2（面積）と(tbps_lv_x（X座標）の平方 + tbps_lv_y（Y座標）の平方 + tbps_lv_z（Z座標）の平方)の平方根の積                                    |
| color_range                    | 色の範囲           | 内部と外部の色相の差の総和            | (tbp_lv_L（明度）- tbp_lv_Lext（外部明度）).abs() + (tbp_lv_A（A）- tbp_lv_Aext（外部A）).abs() + (tbp_lv_B（B）- tbp_lv_Bext（外部B）).abs() |
| shape_color_consistency        | 形状と色の一貫性       | 偏心率と色不規則性の積              | tbp_lv_eccentricity（偏心率）とtbp_lv_color_std_mean（色不規則性）の積                                                                 |
| border_length_ratio            | 境界長比率          | 周囲長と円形の周囲長の比率            | tbp_lv_perimeterMM（周囲長）を(2 * π * sqrt(tbp_lv_areaMM2（面積）/ π))で割る                                                        |
| age_size_symmetry_index        | 年齢、サイズ、対称性指数   | 年齢、最大径、対称性の積             | age_approx（おおよその年齢）とclin_size_long_diam_mm（最大径）とtbp_lv_symm_2axis（対称性）の積                                                |
| **nevi_confidence_area**       | 面積正規化ネビス信頼度    | ネビス信頼度を面積で正規化した値         | tbp_lv_nevi_confidence（ネビス信頼度）をtbp_lv_areaMM2（面積）で割る                                                                    |
| **nevi_confidence_color**      | 色不規則性正規化ネビス信頼度 | ネビス信頼度を色不規則性で割った値        | tbp_lv_nevi_confidence（ネビス信頼度）をtbp_lv_color_std_mean（色不規則性）で割る                                                          |
| **tbp_lv_areaMM3**             | 3D体積近似         | 病変の面積と3D位置座標の距離の積        | 3D位置座標の距離（df['3d_position_distance']）と病変の面積（df['tbp_lv_areaMM2']）の積                                                     |
