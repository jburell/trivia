(ns app.core
    (:require-macros
      [cljs.core.async.macros :as asyncm :refer (go go-loop)])
    (:require [reagent.core :as reagent :refer [atom]]
              [clojure.string :as str]
              [ajax.core :as ajx]
              [cljs.core.async :as async :refer (<! >! put! chan)]
              [taoensso.encore :as encore :refer-macros (have have?)]
              [taoensso.timbre :as timbre :refer-macros (tracef debugf infof warnf errorf)]
              [taoensso.sente :as sente :refer (cb-success?)]
              ;; Optional, for Transit encoding:
              [taoensso.sente.packers.transit :as t]))

(enable-console-print!)

(println "Console ready")

;; define your app data so that it doesn't get over-written on reload

(defonce app-state (atom {:text "Trivia"}))


(let [{:keys [chsk ch-recv send-fn state]}
      (sente/make-channel-socket! "/chsk" ; Note the same path as before
       {:type :auto ; e/o #{:auto :ajax :ws}
       :host "localhost:8080"
       }
       )]
    (def chsk       chsk)
    (def ch-chsk    ch-recv) ; ChannelSocket's receive channel
    (def chsk-send! send-fn) ; ChannelSocket's send API fn
    (def chsk-state state)   ; Watchable, read-only atom
  )


(defn app []
  [:div.center
    [:h1 (:text @app-state)]
    [:h3.question-marker "Q: " [:span.question "What  is...?"]]])

(reagent/render-component [app]
                          (. js/document (getElementById "app")))

(def output-el (.getElementById js/document "output"))
(defn ->output! [fmt & args]
      (let [output-el (.getElementById js/document "output")]
           (if (some? output-el)
             (let [msg (apply encore/format fmt args)]
                  (timbre/debug msg)
                  (aset output-el "value" (str "• " (.-value output-el) "\n" msg))
                  (aset output-el "scrollTop" (.-scrollHeight output-el))))))

(defmulti -event-msg-handler
          "Multimethod to handle Sente `event-msg`s"
          :id                                               ; Dispatch on event-id
          )

(defn event-msg-handler
      "Wraps `-event-msg-handler` with logging, error catching, etc."
      [{:as ev-msg :keys [id ?data event]}]
      (-event-msg-handler ev-msg))

(defmethod -event-msg-handler
           :default                                         ; Default/fallback case (no other matching handler)
           [{:as ev-msg :keys [event]}]
           (->output! "Unhandled event: %s" event))

(defmethod -event-msg-handler :chsk/state
           [{:as ev-msg :keys [?data]}]
           (let [[old-state-map new-state-map] (have vector? ?data)]
                (if (:first-open? new-state-map)
                  (->output! "Channel socket successfully established!: %s" new-state-map)
                  (->output! "Channel socket state change: %s" new-state-map))))

(defmethod -event-msg-handler :chsk/recv
           [{:as ev-msg :keys [?data]}]
           (->output! "Push event from server: %s" ?data))

(defmethod -event-msg-handler :chsk/handshake
          [{:as ev-msg :keys [?data]}]
          (let [[?uid ?csrf-token ?handshake-data] ?data]
            (->output! "Handshake: %s" ?data)))

(defonce router_ (atom nil))
(defn stop-router! [] (when-let [stop-f @router_] (stop-f)))
(defn start-router! []
      (stop-router!)
      (reset! router_
              (sente/start-client-chsk-router!
                ch-chsk event-msg-handler)))
(defn start!
      []
      (start-router!)
      (reagent/render-component [app]
        (.getElementById js/document "app")))

;(defn ^:export run []
;  (reagent/render [app]
;            (js/document.getElementById "app")))        

(defn on-js-reload []
  ;; optionally touch your app-state to force rerendering depending on
  ;; your application
  ;; (swap! app-state update-in [:__figwheel_counter] inc)
)
