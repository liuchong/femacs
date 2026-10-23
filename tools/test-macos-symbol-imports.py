"""Q symbols use function exports; reject accidental data imports before linking."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('exports', Path(__file__).with_name('gen-macos-exports.py'))
exports = importlib.util.module_from_spec(spec)
spec.loader.exec_module(exports)

class SymbolImports(unittest.TestCase):
    def test_function_is_valid(self):
        exports.validate_symbol_imports('@_silgen_name("Qtest") func Qtest() -> Lisp_Object', 'fixture')
    def test_data_import_mutations_are_rejected(self):
        for decl in ['var', 'let', 'private var', 'fileprivate let']:
            with self.subTest(declaration=decl):
                with self.assertRaisesRegex(ValueError, 'Qtest.*function'):
                    exports.validate_symbol_imports('@_silgen_name("Qtest")\n'+decl+' value: Lisp_Object', 'fixture')
    def test_real_data_global_is_allowed(self):
        exports.validate_symbol_imports('@_silgen_name("echo_area_window") var value: Lisp_Object', 'fixture')
    def test_actual_platform_imports(self):
        imports = exports.swift_imports()
        self.assertIn('Qns_in_echo_area', imports)
        self.assertEqual(exports.split_args(imports['Qns_in_echo_area']['args']), [])

if __name__ == '__main__': unittest.main()
