import 'package:integration_test/integration_test.dart';

import 'board_row_test.dart' as board_row_test;
import 'board_add_row_test.dart' as board_add_row_test;

void startTesting() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  // Board integration tests
  board_row_test.main();
  board_add_row_test.main();
}
