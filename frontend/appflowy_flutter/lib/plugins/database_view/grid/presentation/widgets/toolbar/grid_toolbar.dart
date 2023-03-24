import 'package:appflowy/plugins/database_view/application/field/field_controller.dart';
import 'package:flutter/material.dart';

import '../../layout/sizes.dart';
import 'filter_button.dart';
import 'setting_button.dart';
import 'sort_button.dart';

class GridToolbarContext {
  final String viewId;
  final FieldController fieldController;
  GridToolbarContext({
    required this.viewId,
    required this.fieldController,
  });
}

class GridToolbar extends StatelessWidget {
  const GridToolbar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          SizedBox(width: GridSize.leadingHeaderPadding),
          const Spacer(),
          const FilterButton(),
          const SortButton(),
          const SettingButton(),
        ],
      ),
    );
  }
}
