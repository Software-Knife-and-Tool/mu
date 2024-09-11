;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(mu:intern (mu:find-namespace "") "%profile%" (mu:make-namespace "%profile%"))

(mu:intern %profile% "profile"
   (:lambda (src-file output-file)
     (prelude:load src-file :t)
     (prof:prof-control :on)
     (%prof-src%:run-prof)
     (prof:prof-control :off)
      ((:lambda (stream)
          (core:%vector-map
           (:lambda (lambda-count)
             (core:format stream "~A~T~A~%"
                          `(,(mu:car lambda-count)
                             ,(mu:cdr lambda-count))))
           (prof:prof-control :get))
         (mu:close stream))
       (mu:open :file :output output-file))
      :t))
