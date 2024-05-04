;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(lib:intern (lib:find-ns "prelude") "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (lib:boundp symbol)
                   (prelude:format stream "~A~T~A~T~S~%"
                                `(,(lib:symbol-name symbol)
                                  ,(prelude:type-of (lib:symbol-value symbol))
                                  ,(lib:symbol-value symbol)))
                   (prelude:format stream "unbound~Tunbound~T~A~%" `(,(lib:symbol-name symbol)))))
           (lib:symbols (lib:find-ns "prelude"))))
       (lib:open :file :output file-name))
      :t))
