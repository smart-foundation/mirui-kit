#!/usr/bin/env python3
"""Exercise visual regression detection without needing the native toolchain."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
spec=importlib.util.spec_from_file_location('visual',Path(__file__).with_name('compare-native-captures.py'))
visual=importlib.util.module_from_spec(spec);spec.loader.exec_module(visual)
class Comparison(unittest.TestCase):
    def test_single_pixel_regression_is_not_hidden(self):
        with tempfile.TemporaryDirectory() as folder:
            a,b,d=[Path(folder)/name for name in ('a.png','b.png','diff.png')]
            pixels=bytearray([80,90,100,255]*12)
            visual.write_rgba(a,4,3,pixels)
            self.assertEqual(visual.rgba(a),(4,3,pixels))
            visual.write_rgba(b,4,3,pixels)
            self.assertEqual(visual.compare(a,b,d)['status'],'identical')
            pixels[4*6]=200;visual.write_rgba(b,4,3,pixels)
            result=visual.compare(a,b,d)
            self.assertEqual(result['changed_pixels'],1)
            self.assertEqual(result['difference_bounds'],[2,1,2,1])
            self.assertTrue(d.exists())
    def test_size_change_is_a_regression(self):
        with tempfile.TemporaryDirectory() as folder:
            a,b,d=[Path(folder)/name for name in ('a.png','b.png','diff.png')]
            visual.write_rgba(a,1,1,bytes([0,0,0,255]))
            visual.write_rgba(b,2,1,bytes([0,0,0,255]*2))
            self.assertEqual(visual.compare(a,b,d)['status'],'changed-size')
    def test_corrupt_capture_cannot_be_reviewed_as_valid(self):
        with tempfile.TemporaryDirectory() as folder:
            path=Path(folder)/'bad.png';visual.write_rgba(path,1,1,bytes([0,0,0,255]))
            data=bytearray(path.read_bytes());data[20]^=1;path.write_bytes(data)
            with self.assertRaises(ValueError):visual.rgba(path)
if __name__=='__main__':unittest.main()
