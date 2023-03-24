import 'package:appflowy/plugins/database_view/grid/application/grid_bloc.dart';
import 'package:appflowy/plugins/database_view/application/database_controller.dart';
import 'package:appflowy_backend/protobuf/flowy-database/setting_entities.pbenum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bloc_test/bloc_test.dart';
import 'util.dart';

void main() {
  late AppFlowyGridTest gridTest;
  setUpAll(() async {
    gridTest = await AppFlowyGridTest.ensureInitialized();
  });

  group('Edit Grid:', () {
    late GridTestContext context;
    setUp(() async {
      context = await gridTest.createTestGrid();
    });
    // The initial number of rows is 3 for each grid.
    blocTest<GridBloc, GridState>(
      "create a row",
      build: () => GridBloc(
          view: context.gridView,
          databaseController: DatabaseController(
            view: context.gridView,
            layoutType: LayoutTypePB.Grid,
          ))
        ..add(const GridEvent.initial()),
      act: (bloc) => bloc.add(const GridEvent.createRow()),
      wait: const Duration(milliseconds: 300),
      verify: (bloc) {
        assert(bloc.state.rowInfos.length == 4);
      },
    );

    blocTest<GridBloc, GridState>(
      "delete the last row",
      build: () => GridBloc(
          view: context.gridView,
          databaseController: DatabaseController(
            view: context.gridView,
            layoutType: LayoutTypePB.Grid,
          ))
        ..add(const GridEvent.initial()),
      act: (bloc) async {
        await gridResponseFuture();
        bloc.add(GridEvent.deleteRow(bloc.state.rowInfos.last));
      },
      wait: const Duration(milliseconds: 300),
      verify: (bloc) {
        assert(bloc.state.rowInfos.length == 2,
            "Expected 2, but receive ${bloc.state.rowInfos.length}");
      },
    );
  });
}
