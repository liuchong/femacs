class Femacs < Formula
  desc "Standalone No-GIL Elisp-compatible CLI (elisp) and Fe editor CLI launchers"
  homepage "https://femacs.com/"
  version "0.0.1"
  license "GPL-3.0-or-later"

  on_macos do
    on_arm do
      url "https://github.com/liuchong/femacs/releases/download/v0.0.1/femacs-v0.0.1-aarch64-apple-darwin.tar.gz"
      sha256 "f42a8fddfe5fbd81b90c5146c7824cdd27c7b4af41f8f389ce4103d214d37700"
    end

    on_intel do
      url "https://github.com/liuchong/femacs/releases/download/v0.0.1/femacs-v0.0.1-x86_64-apple-darwin.tar.gz"
      sha256 "d341fdca2c411fc1583243ba965d3f360fb74ebdbf25860a5fd7fa843da0c6b0"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/liuchong/femacs/releases/download/v0.0.1/femacs-v0.0.1-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "b2d6965e261d13bc0030d5c8c923b8361888caf231b154c4cb897d141aa579f6"
    end

    on_intel do
      url "https://github.com/liuchong/femacs/releases/download/v0.0.1/femacs-v0.0.1-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "2678a3a76bb4dfe96e4be9a86094d1223e6f09003b7dc54e95ba8bfe97311bca"
    end
  end

  def install
    bin.install "elisp", "femacs", "fe"
  end

  test do
    assert_match "elisp #{version}", shell_output("#{bin}/elisp --version")
    assert_equal "42", shell_output("#{bin}/elisp --no-auto-warm eval '(+ 20 22)'").strip
  end
end
